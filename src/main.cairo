%builtins output pedersen range_check
from starkware.cairo.common.hash_state import hash_finalize, hash_init, hash_update
from starkware.cairo.common.cairo_builtins import HashBuiltin
from starkware.cairo.common.registers import get_fp_and_pc

func main{output_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr: felt*}() -> () {
    alloc_locals;
    let (__fp__, _) = get_fp_and_pc();

    // Load fibonacci_claim_index and copy it to the output segment.
    local password: felt*;
    local password_len: felt;
    local value: felt;
    %{
        program_input_password = program_input["password"]

        password = [
            int(password_char)
            for password_char in program_input_password
        ]
        ids.password = segments.gen_arg(password)
        ids.password_len = len(password)
        ids.value = program_input["value"]
    %}

    // Main page hash.
    let (hash_state_ptr) = hash_init();
    let (hash_state_ptr) = hash_update{hash_ptr=pedersen_ptr}(
        hash_state_ptr=hash_state_ptr,
        data_ptr=password,
        data_length=password_len,
    );
    let (password_hash) = hash_finalize{hash_ptr=pedersen_ptr}(hash_state_ptr=hash_state_ptr);

    assert output_ptr[0] = password_hash;
    assert output_ptr[1] = value;
    let output_ptr = output_ptr + 2;

    // Return the updated output_ptr.
    return ();
}

func fib(first_element: felt, second_element: felt, n: felt) -> felt {
    if (n == 0) {
        return second_element;
    }

    return fib(
        first_element=second_element, second_element=first_element + second_element, n=n - 1
    );
}
