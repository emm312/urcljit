use std::{
    collections::HashMap,
    thread::{self, JoinHandle}, time::Instant,
};

use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    execution_engine::{ExecutionEngine, JitFunction},
    module::Module,
    types::{BasicMetadataTypeEnum, IntType},
    values::{BasicMetadataValueEnum, IntValue, BasicValue},
    AddressSpace, OptimizationLevel,
};

use crate::opcodes::{LabelHash, Opcode, Operand, Port, Program, Register};

pub fn llvm_jit(ast: Vec<Program>, headers: (usize, usize, usize, usize)) -> JoinHandle<()> {
    thread::spawn(move || {
        Codegen::jit_compile(ast, headers);
    })
}

pub type UrclFunc = unsafe extern "C" fn();

struct Codegen<'ctx> {
    word: IntType<'ctx>,

    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    exec_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> Codegen<'ctx> {
    pub fn jit_compile(ast: Vec<Program>, headers: (usize, usize, usize, usize)) {
        println!("started JIT compiling the AST");
        let start = Instant::now();
        let context = Context::create();
        let module = context.create_module("urcl");
        let exec_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();
        let integer_type = context.custom_width_int_type(headers.0 as u32);
        let mut codegen = Codegen {
            word: integer_type,

            context: &context,
            module,
            builder: context.create_builder(),
            exec_engine,
        };

        let func = codegen.jit_compile_urcl(ast, headers);

        println!("finished JIT compiling the AST at {}ms", start.elapsed().as_millis());
        unsafe { func.call() };
    }

    fn jit_compile_urcl(
        &mut self,
        ast: Vec<Program>,
        headers: (usize, usize, usize, usize),
    ) -> JitFunction<'ctx, UrclFunc> {
        let main = self
            .module
            .add_function("main", self.word.fn_type(&[], false), None);
        let basic_block = self.context.append_basic_block(main, "entry");
        self.builder.position_at_end(basic_block);

        let mut registers = vec![self.builder.build_alloca(self.word, "r0").unwrap()];
        for i in 1..headers.0 {
            registers.push(
                self.builder
                    .build_alloca(self.word, format!("r{}", i).as_str())
                    .unwrap(),
            );
        }

        let sp = self.builder.build_alloca(self.word, "sp").unwrap();

        let get_reg = |reg: Register| match reg {
            Register::Sp => sp,
            Register::Gpr(n) => registers[n as usize],
        };

        let blocks: HashMap<LabelHash, BasicBlock<'ctx>> = ast
            .iter()
            .filter(|instr| matches!(instr, Program::Label(_)))
            .map(|label| {
                let label = match label {
                    Program::Label(label) => label,
                    _ => unreachable!(),
                };
                let block = self
                    .context
                    .append_basic_block(main, format!("{}", label).as_str());
                (*label, block)
            })
            .collect();

        let unwrap_operand = |operand: Operand| match operand {
            Operand::Register(reg) => self
                .builder
                .build_load(self.word, get_reg(reg), "loadreg")
                .unwrap()
                .into_int_value(),
            Operand::Immediate(imm) => self.word.const_int(imm as u64, false),
            Operand::Label(_) => {
                todo!()
            }
        };

        for node in ast {
            match node {
                Program::Instruction(instr) => {
                    let to_write_to = match instr.yielded {
                        Some(Register::Gpr(n)) => {
                            if n != 0 {
                                Some(registers[n as usize])
                            } else {
                                None
                            }
                        }
                        Some(Register::Sp) => Some(sp),
                        None => None,
                    };

                    let mut instr_res = None;

                    match instr.opcode {
                        Opcode::Add(src1, src2) => {
                            let src1 = unwrap_operand(src1);
                            let src2 = unwrap_operand(src2);
                            instr_res = Some(self.builder.build_int_add(src1, src2, "add").unwrap());
                        }
                        Opcode::Out(port, reg) => match port {
                            Port::Text => {
                                let val = unwrap_operand(reg);
                                self.build_indirect_call(crate::io::putc, val)
                            }
                            _ => todo!(),
                        },
                        _ => todo!(),
                    }

                    if let Some(res) = instr_res {
                        if let Some(to_write_to) = to_write_to {
                            self.builder.build_store(to_write_to, res.as_basic_value_enum()).unwrap();
                        }
                    }
                }
                Program::Label(label) => {
                    self.builder.build_unconditional_branch(blocks[&label]).unwrap();
                    self.builder.position_at_end(blocks[&label]);
                }
            }
        }
        self.builder.build_return(None).unwrap();
        unsafe { self.exec_engine.get_function::<UrclFunc>("main").unwrap() }
    }

    fn build_indirect_call(&self, func: extern "C" fn(u64), arg: IntValue<'ctx>) {
        let ptr = unsafe { std::mem::transmute::<extern "C" fn(u64), *const u8>(func) } as u64;
        let ll_ptr = self.context.i64_type().const_int(ptr, false);
        let function_pointer = self
            .builder
            .build_int_to_ptr(
                ll_ptr,
                self.context.i64_type().ptr_type(AddressSpace::default()),
                "function_pointer",
            )
            .unwrap();
        let func_ty = self.context.void_type().fn_type(
            &[BasicMetadataTypeEnum::IntType(self.context.i128_type())],
            false,
        );
        self.builder
            .build_indirect_call(
                func_ty,
                function_pointer,
                &[BasicMetadataValueEnum::IntValue(arg)],
                "indirect call to port",
            )
            .unwrap();
    }
}
