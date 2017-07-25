use vm::local_variables::LocalVariables;
use vm::stack::Stack;
use vm::stack::StackEntry;

#[repr(u8)]
enum ByteCode {
    aload_0 = 0x2A,
    bipush = 0x10,
    dup = 0x59,
    getstatic = 0xB2,
    iadd = 0x60,
    iconst_0 = 0x03,
    iconst_1 = 0x04,
    iconst_2 = 0x05,
    iconst_3 = 0x06,
    iconst_4 = 0x07,
    iconst_5 = 0x08,
    idiv = 0x6C,
    imul = 0x68,
    invokespecial = 0xB7,
    invokevirtual = 0xB6,
    iload = 0x15,
    iload_1 = 0x1B,
    iload_2 = 0x1C,
    iload_3 = 0x1D,
    istore = 0x36,
    istore_1 = 0x3C,
    istore_2 = 0x3D,
    istore_3 = 0x3E,
    isub = 0x64,
    ldc = 0x12,
    op_new = 0xBB,
    return_op = 0xB1,
}



fn iload(stack: &mut Stack, local_variables: &mut LocalVariables, next_byte: u8) -> u16 {
    let index = next_byte;
    let value = local_variables.get(index);

    debug!("iload: load value from local variable {}({})", index, value);

    stack.push_imm(*value);

    return 2;
}

fn iload_1(stack: &mut Stack, local_variables: &mut LocalVariables) -> u16 {
    let value = local_variables.get(1);

    debug!("iload_1: load value from local variable 1({})", value);

    stack.push_imm(*value);
    return 1;
}

fn iload_2(stack: &mut Stack, local_variables: &mut LocalVariables) -> u16 {
    let value = local_variables.get(2);

    debug!("iload_2: load value from local variable 2({})", value);

    stack.push_imm(*value);
    return 1;
}

fn iload_3(stack: &mut Stack, local_variables: &mut LocalVariables) -> u16 {
    let value = local_variables.get(3);

    debug!("iload_3: load value from local variable 3({})", value);

    stack.push_imm(*value);
    return 1;
}

fn istore(stack: &mut Stack, local_variables: &mut LocalVariables, nextByte: u8) -> i16 {
    let value = stack.pop_imm();
    let index = nextByte;

    debug!("istore: store value into local variable {}({})", index, value);

    *local_variables.get(index) = value;
    return 2;
}

fn istore_1(stack: &mut Stack, local_variables: &mut LocalVariables) -> u16 {
    let value = stack.pop_imm();

    debug!("istore_1: store value into local variable 1({})", value);

    *local_variables.get(1) = value;
    return 1;
}

fn istore_2(stack: &mut Stack, local_variables: &mut LocalVariables) -> u16 {
    let value = stack.pop_imm();

    debug!("istore_2: store value into local variable 2({})", value);

    *local_variables.get(2) = value;
    return 1;
}

fn istore_3(stack: &mut Stack, local_variables: &mut LocalVariables) -> u16 {
    let value = stack.pop_imm();

    debug!("istore_3: store value into local variable 3({})", value);

    *local_variables.get(3) = value;
    return 1;
}

fn isub(stack: &mut Stack, local_variables: &mut LocalVariables) -> u16 {
    let value2 = stack.pop_imm();
    let value1 = stack.pop_imm();
    let result = value1 - value2;

    debug!("isub : {} - {} = {}", value1, value2, result);

    stack.push_imm(result);
    return 1;
}

fn idiv(stack: &mut Stack, local_variables: &mut LocalVariables) -> u16 {
    let value2 = stack.pop_imm();
    let value1 = stack.pop_imm();
    let result = value1 / value2;
    stack.push_imm(result);

    debug!("idiv: {} / {} = {}", value1, value2, result);

    return 1;
}

fn imul(stack: &mut Stack, local_variables: &mut LocalVariables) -> u16 {
    let value1 = stack.pop_imm();
    let value2 = stack.pop_imm();
    let result = value1 * value2;
    stack.push_imm(result);

    debug!("imul : {} * {} = {}", value1, value2, result);

    return 1;
}

fn iadd(stack: &mut Stack, local_variables: &mut LocalVariables) -> u16 {
    let value1 = stack.pop_imm();
    let value2 = stack.pop_imm();
    let result = value1 + value2;

    debug!("iadd: {} + {} = {}\n", value1, value2, result);

    stack.push_imm(result);
    return 1;
}


fn bipush(stack: &mut Stack, local_variables: &mut LocalVariables, nextByte: u8) -> i16 {
    let value = nextByte;
    stack.push_imm(value as u32);

    debug!("push a byte {} onto the stack \n", value);

    return 2;
}

fn dup(stack: &mut Stack, local_variables: &mut LocalVariables, nextByte: u8) -> i16 {
    if let Some(entry) = stack.0.pop() {
        stack.0.push(entry.clone());
        stack.0.push(entry.clone());

        match entry {
            StackEntry::Reference(_) => debug!("dup: duplicating stack reference"),
            StackEntry::Immediate(_) => debug!("dup: duplicating stack immediate"),
        }
    } else {
        panic!("Empty Stack!");
    }
    return 1;
}

fn ldc(stack: &mut Stack, local_variables: &mut LocalVariables, nextByte: u8) -> i16 {
    let value = nextByte;
    stack.push_ref(value as u32);

    debug!("ldc: push a constant index {} onto the stack ", value);

    return 2;
}

fn op_return(stack: &mut Stack, local_variables: &mut LocalVariables, nextByte: u8) -> i16 {
    debug!("return ");

    return -1;
}

fn aload_0(stack: &mut Stack, local_variables: &mut LocalVariables, nextByte: u8) -> i16 {
    stack.push_imm(0);


    debug!("aload_0: push 0 to stack\n");

    return 1;
}

fn iconst_0(stack: &mut Stack, local_variables: &mut LocalVariables, nextByte: u8) -> i16 {
    stack.push_imm(0);

    debug!("iconst_0: push 0 to stack\n");

    return 1;
}

fn iconst_1(stack: &mut Stack, local_variables: &mut LocalVariables, nextByte: u8) -> i16 {
    stack.push_imm(1);

    debug!("iconst_1: push 1 to stack\n");

    return 1;
}

fn iconst_2(stack: &mut Stack, local_variables: &mut LocalVariables, nextByte: u8) -> i16 {
    stack.push_imm(2);

    debug!("iconst_2: push 2 to stack\n");

    return 1;
}

fn iconst_3(stack: &mut Stack, local_variables: &mut LocalVariables, nextByte: u8) -> i16 {
    stack.push_imm(3);

    debug!("iconst_3: push 3 to stack\n");

    return 1;
}

fn iconst_4(stack: &mut Stack, local_variables: &mut LocalVariables, nextByte: u8) -> i16 {
    stack.push_imm(4);

    debug!("iconst_4: push 4 to stack\n");

    return 1;
}

fn iconst_5(stack: &mut Stack, local_variables: &mut LocalVariables, nextByte: u8) -> i16 {
    stack.push_imm(5);

    debug!("iconst_5: push 5 to stack\n");

    return 1;
}