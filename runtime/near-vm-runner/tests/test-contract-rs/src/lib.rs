#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use std::mem::size_of;

#[allow(unused)]
extern "C" {
    // #############
    // # Registers #
    // #############
    fn read_register(register_id: u64, ptr: u64);
    fn register_len(register_id: u64) -> u64;
    // ###############
    // # Context API #
    // ###############
    fn current_account_id(register_id: u64);
    fn signer_account_id(register_id: u64);
    fn signer_account_pk(register_id: u64);
    fn predecessor_account_id(register_id: u64);
    fn input(register_id: u64);
    fn block_index() -> u64;
    fn storage_usage() -> u64;
    // #################
    // # Economics API #
    // #################
    fn account_balance(balance_ptr: u64);
    fn attached_deposit(balance_ptr: u64);
    fn prepaid_gas() -> u64;
    fn used_gas() -> u64;
    // ############
    // # Math API #
    // ############
    fn random_seed(register_id: u64);
    fn sha256(value_len: u64, value_ptr: u64, register_id: u64);
    // #####################
    // # Miscellaneous API #
    // #####################
    fn value_return(value_len: u64, value_ptr: u64);
    fn panic();
    fn log_utf8(len: u64, ptr: u64);
    fn log_utf16(len: u64, ptr: u64);
    fn abort(msg_ptr: u32, filename_ptr: u32, line: u32, col: u32);
    // ################
    // # Promises API #
    // ################
    fn promise_create(
        account_id_len: u64,
        account_id_ptr: u64,
        method_name_len: u64,
        method_name_ptr: u64,
        arguments_len: u64,
        arguments_ptr: u64,
        amount_ptr: u64,
        gas: u64,
    ) -> u64;
    fn promise_then(
        promise_index: u64,
        account_id_len: u64,
        account_id_ptr: u64,
        method_name_len: u64,
        method_name_ptr: u64,
        arguments_len: u64,
        arguments_ptr: u64,
        amount_ptr: u64,
        gas: u64,
    ) -> u64;
    fn promise_and(promise_idx_ptr: u64, promise_idx_count: u64) -> u64;
    fn promise_batch_create(account_id_len: u64, account_id_ptr: u64) -> u64;
    fn promise_batch_then(promise_index: u64, account_id_len: u64, account_id_ptr: u64) -> u64;
    // #######################
    // # Promise API actions #
    // #######################
    fn promise_batch_action_create_account(promise_index: u64);
    fn promise_batch_action_deploy_contract(promise_index: u64, code_len: u64, code_ptr: u64);
    fn promise_batch_action_function_call(
        promise_index: u64,
        method_name_len: u64,
        method_name_ptr: u64,
        arguments_len: u64,
        arguments_ptr: u64,
        amount_ptr: u64,
        gas: u64,
    );
    fn promise_batch_action_transfer(promise_index: u64, amount_ptr: u64);
    fn promise_batch_action_stake(
        promise_index: u64,
        amount_ptr: u64,
        public_key_len: u64,
        public_key_ptr: u64,
    );
    fn promise_batch_action_add_key_with_full_access(
        promise_index: u64,
        public_key_len: u64,
        public_key_ptr: u64,
        nonce: u64,
    );
    fn promise_batch_action_add_key_with_function_call(
        promise_index: u64,
        public_key_len: u64,
        public_key_ptr: u64,
        nonce: u64,
        allowance_ptr: u64,
        receiver_id_len: u64,
        receiver_id_ptr: u64,
        method_names_len: u64,
        method_names_ptr: u64,
    );
    fn promise_batch_action_delete_key(
        promise_index: u64,
        public_key_len: u64,
        public_key_ptr: u64,
    );
    fn promise_batch_action_delete_account(
        promise_index: u64,
        beneficiary_id_len: u64,
        beneficiary_id_ptr: u64,
    );
    // #######################
    // # Promise API results #
    // #######################
    fn promise_results_count() -> u64;
    fn promise_result(result_idx: u64, register_id: u64) -> u64;
    fn promise_return(promise_id: u64);
    // ###############
    // # Storage API #
    // ###############
    fn storage_write(
        key_len: u64,
        key_ptr: u64,
        value_len: u64,
        value_ptr: u64,
        register_id: u64,
    ) -> u64;
    fn storage_read(key_len: u64, key_ptr: u64, register_id: u64) -> u64;
    fn storage_remove(key_len: u64, key_ptr: u64, register_id: u64) -> u64;
    fn storage_has_key(key_len: u64, key_ptr: u64) -> u64;
    fn storage_iter_prefix(prefix_len: u64, prefix_ptr: u64) -> u64;
    fn storage_iter_range(start_len: u64, start_ptr: u64, end_len: u64, end_ptr: u64) -> u64;
    fn storage_iter_next(iterator_id: u64, key_register_id: u64, value_register_id: u64) -> u64;
}

#[no_mangle]
pub fn write_key_value() {
    unsafe {
        input(0);
        if register_len(0) != 2 * size_of::<u64>() as u64 {
            panic()
        }
        let data = [0u8; 2 * size_of::<u64>()];
        read_register(0, data.as_ptr() as u64);

        let key = &data[0..size_of::<u64>()];
        let value = &data[size_of::<u64>()..];
        let result = storage_write(
            key.len() as u64,
            key.as_ptr() as u64,
            value.len() as u64,
            value.as_ptr() as u64,
            1,
        );
        value_return(size_of::<u64>() as u64, &result as *const u64 as u64);
    }
}

#[no_mangle]
pub fn read_value() {
    unsafe {
        input(0);
        if register_len(0) != size_of::<u64>() as u64 {
            panic()
        }
        let key = [0u8; size_of::<u64>()];
        read_register(0, key.as_ptr() as u64);
        let result = storage_read(key.len() as u64, key.as_ptr() as u64, 1);
        if result == 1 {
            let value = [0u8; size_of::<u64>()];
            read_register(1, value.as_ptr() as u64);
            value_return(value.len() as u64, &value as *const u8 as u64);
        }
    }
}

#[no_mangle]
pub fn log_something() {
    unsafe {
        let data = b"hello";
        log_utf8(data.len() as u64, data.as_ptr() as _);
    }
}

#[no_mangle]
pub fn run_test() {
    unsafe {
        let value: [u8; 4] = 10i32.to_le_bytes();
        value_return(value.len() as u64, value.as_ptr() as _);
    }
}

#[no_mangle]
pub fn sum_with_input() {
    unsafe {
        input(0);
        if register_len(0) != 2 * size_of::<u64>() as u64 {
            panic()
        }
        let data = [0u8; 2 * size_of::<u64>()];
        read_register(0, data.as_ptr() as u64);

        let mut key = [0u8; size_of::<u64>()];
        let mut value = [0u8; size_of::<u64>()];
        key.copy_from_slice(&data[..size_of::<u64>()]);
        value.copy_from_slice(&data[size_of::<u64>()..]);
        let key = u64::from_le_bytes(key);
        let value = u64::from_le_bytes(value);
        let result = key + value;
        value_return(size_of::<u64>() as u64, &result as *const u64 as u64);
    }
}

/// Writes and reads some data into/from storage. Uses 8-bit key/values.
#[no_mangle]
pub fn benchmark_storage_8b() {
    unsafe {
        input(0);
        if register_len(0) != size_of::<u64>() as u64 {
            panic()
        }
        let data = [0u8; size_of::<u64>()];
        read_register(0, data.as_ptr() as u64);
        let n: u64 = u64::from_le_bytes(data);

        let mut sum = 0u64;
        for i in 0..n {
            let el = i.to_le_bytes();
            storage_write(
                el.len() as u64,
                el.as_ptr() as u64,
                el.len() as u64,
                el.as_ptr() as u64,
                0,
            );

            let result = storage_read(el.len() as u64, el.as_ptr() as u64, 0);
            if result == 1 {
                let value = [0u8; size_of::<u64>()];
                read_register(0, value.as_ptr() as u64);
                sum += u64::from_le_bytes(value);
            }
        }

        value_return(size_of::<u64>() as u64, &sum as *const u64 as u64);
    }
}

#[inline]
fn generate_data(data: &mut [u8]) {
    for i in 0..data.len() {
        data[i] = (i % std::u8::MAX as usize) as u8;
    }
}

/// Writes and reads some data into/from storage. Uses 10KiB key/values.
#[no_mangle]
pub fn benchmark_storage_10kib() {
    unsafe {
        input(0);
        if register_len(0) != size_of::<u64>() as u64 {
            panic()
        }
        let data = [0u8; size_of::<u64>()];
        read_register(0, data.as_ptr() as u64);
        let n: u64 = u64::from_le_bytes(data);

        let mut el = [0u8; 10 << 10];
        generate_data(&mut el);

        let mut sum = 0u64;
        for i in 0..n {
            el[..size_of::<u64>()].copy_from_slice(&i.to_le_bytes());
            storage_write(
                el.len() as u64,
                el.as_ptr() as u64,
                el.len() as u64,
                el.as_ptr() as u64,
                0,
            );

            let result = storage_read(el.len() as u64, el.as_ptr() as u64, 0);
            if result == 1 {
                let el = [0u8; 10 << 10];
                read_register(0, el.as_ptr() as u64);
                let mut value = [0u8; size_of::<u64>()];
                value.copy_from_slice(&el[0..size_of::<u64>()]);
                sum += u64::from_le_bytes(value);
            }
        }

        value_return(size_of::<u64>() as u64, &sum as *const u64 as u64);
    }
}

/// Passes through input into output.
#[no_mangle]
pub fn pass_through() {
    unsafe {
        input(0);
        if register_len(0) != size_of::<u64>() as u64 {
            panic()
        }
        let data = [0u8; size_of::<u64>()];
        read_register(0, data.as_ptr() as u64);
        value_return(data.len() as u64, data.as_ptr() as u64);
    }
}

/// Sums numbers.
#[no_mangle]
pub fn sum_n() {
    unsafe {
        input(0);
        if register_len(0) != size_of::<u64>() as u64 {
            panic()
        }
        let data = [0u8; size_of::<u64>()];
        read_register(0, data.as_ptr() as u64);
        let n = u64::from_le_bytes(data);

        let mut sum = 0u128;
        for i in 1..n + 1 {
            sum += (i * i) as u128;
        }
        sum /= n as u128;

        let data = (sum as u64).to_le_bytes();
        value_return(data.len() as u64, data.as_ptr() as u64);
    }
}

#[no_mangle]
pub fn insert_strings() {
    unsafe {
        input(0);
        if register_len(0) != 2 * size_of::<u64>() as u64 {
            panic()
        }
        let data = [0u8; 2 * size_of::<u64>()];
        read_register(0, data.as_ptr() as u64);

        let mut from = [0u8; size_of::<u64>()];
        let mut to = [0u8; size_of::<u64>()];
        from.copy_from_slice(&data[..size_of::<u64>()]);
        to.copy_from_slice(&data[size_of::<u64>()..]);
        let from = u64::from_le_bytes(from);
        let to = u64::from_le_bytes(to);
        let s = vec![b'a'; to as usize];
        for i in from..to {
            let mut key = s[(to - i) as usize..].to_vec();
            key.push(b'b');
            let value = b"x";
            storage_write(
                key.len() as u64,
                key.as_ptr() as u64,
                value.len() as u64,
                value.as_ptr() as u64,
                0,
            );
        }
    }
}

#[no_mangle]
pub fn delete_strings() {
    unsafe {
        input(0);
        if register_len(0) != 2 * size_of::<u64>() as u64 {
            panic()
        }
        let data = [0u8; 2 * size_of::<u64>()];
        read_register(0, data.as_ptr() as u64);

        let mut from = [0u8; size_of::<u64>()];
        let mut to = [0u8; size_of::<u64>()];
        from.copy_from_slice(&data[..size_of::<u64>()]);
        to.copy_from_slice(&data[size_of::<u64>()..]);
        let from = u64::from_le_bytes(from);
        let to = u64::from_le_bytes(to);
        let s = vec![b'a'; to as usize];
        for i in from..to {
            let mut key = s[(to - i) as usize..].to_vec();
            key.push(b'b');
            storage_remove(key.len() as u64, key.as_ptr() as u64, 0);
        }
    }
}

#[no_mangle]
pub fn recurse() {
    unsafe {
        input(0);
        if register_len(0) != size_of::<u64>() as u64 {
            panic()
        }
        let data = [0u8; size_of::<u64>()];
        read_register(0, data.as_ptr() as u64);
        let n = u64::from_le_bytes(data);
        let res = internal_recurse(n);
        let data = res.to_le_bytes();
        value_return(data.len() as u64, data.as_ptr() as u64);
    }
}

#[no_mangle]
fn internal_recurse(n: u64) -> u64 {
    if n == 0 {
        n
    } else {
        internal_recurse(n - 1) + 1
    }
}

// Can be used for debugging
#[no_mangle]
fn log_u64(msg: u64) {
    unsafe {
        log_utf8(8, &msg as *const u64 as u64);
    }
}

pub fn from_base64(s: &str) -> Vec<u8> {
    base64::decode(s).unwrap()
}

#[no_mangle]
fn call_promise() {
    unsafe {
        input(0);
        let data = vec![0u8; register_len(0) as usize];
        read_register(0, data.as_ptr() as u64);
        let input_args: serde_json::Value = serde_json::from_slice(&data).unwrap();
        for arg in input_args.as_array().unwrap() {
            let actual_id = if let Some(create) = arg.get("create") {
                let account_id = create["account_id"].as_str().unwrap().as_bytes();
                let method_name = create["method_name"].as_str().unwrap().as_bytes();
                let arguments = serde_json::to_vec(&create["arguments"]).unwrap();
                let amount = create["amount"].as_i64().unwrap() as u128;
                let gas = create["gas"].as_i64().unwrap() as u64;
                promise_create(
                    account_id.len() as u64,
                    account_id.as_ptr() as u64,
                    method_name.len() as u64,
                    method_name.as_ptr() as u64,
                    arguments.len() as u64,
                    arguments.as_ptr() as u64,
                    &amount as *const u128 as *const u64 as u64,
                    gas,
                )
            } else if let Some(then) = arg.get("then") {
                let promise_index = then["promise_index"].as_i64().unwrap() as u64;
                let account_id = then["account_id"].as_str().unwrap().as_bytes();
                let method_name = then["method_name"].as_str().unwrap().as_bytes();
                let arguments = serde_json::to_vec(&then["arguments"]).unwrap();
                let amount = then["amount"].as_i64().unwrap() as u128;
                let gas = then["gas"].as_i64().unwrap() as u64;
                promise_then(
                    promise_index,
                    account_id.len() as u64,
                    account_id.as_ptr() as u64,
                    method_name.len() as u64,
                    method_name.as_ptr() as u64,
                    arguments.len() as u64,
                    arguments.as_ptr() as u64,
                    &amount as *const u128 as *const u64 as u64,
                    gas,
                )
            } else if let Some(and) = arg.get("and") {
                let and = and.as_array().unwrap();
                let mut curr = and[0].as_i64().unwrap() as u64;
                for other in &and[1..] {
                    curr = promise_and(curr, other.as_i64().unwrap() as u64);
                }
                curr
            } else if let Some(batch_create) = arg.get("batch_create") {
                let account_id = batch_create["account_id"].as_str().unwrap().as_bytes();
                promise_batch_create(account_id.len() as u64, account_id.as_ptr() as u64)
            } else if let Some(batch_then) = arg.get("batch_then") {
                let promise_index = batch_then["promise_index"].as_i64().unwrap() as u64;
                let account_id = batch_then["account_id"].as_str().unwrap().as_bytes();
                promise_batch_then(
                    promise_index,
                    account_id.len() as u64,
                    account_id.as_ptr() as u64,
                )
            } else if let Some(action) = arg.get("action_create_account") {
                let promise_index = action["promise_index"].as_i64().unwrap() as u64;
                promise_batch_action_create_account(promise_index);
                promise_index
            } else if let Some(action) = arg.get("action_deploy_contract") {
                let promise_index = action["promise_index"].as_i64().unwrap() as u64;
                let code = from_base64(action["code"].as_str().unwrap());
                promise_batch_action_deploy_contract(
                    promise_index,
                    code.len() as u64,
                    code.as_ptr() as u64,
                );
                promise_index
            } else if let Some(action) = arg.get("action_function_call") {
                let promise_index = action["promise_index"].as_i64().unwrap() as u64;
                let method_name = action["method_name"].as_str().unwrap().as_bytes();
                let arguments = serde_json::to_vec(&action["arguments"]).unwrap();
                let amount = action["amount"].as_i64().unwrap() as u128;
                let gas = action["gas"].as_i64().unwrap() as u64;
                promise_batch_action_function_call(
                    promise_index,
                    method_name.len() as u64,
                    method_name.as_ptr() as u64,
                    arguments.len() as u64,
                    arguments.as_ptr() as u64,
                    &amount as *const u128 as *const u64 as u64,
                    gas,
                );
                promise_index
            } else if let Some(action) = arg.get("action_transfer") {
                let promise_index = action["promise_index"].as_i64().unwrap() as u64;
                let amount = action["amount"].as_i64().unwrap() as u128;
                promise_batch_action_transfer(
                    promise_index,
                    &amount as *const u128 as *const u64 as u64,
                );
                promise_index
            } else if let Some(action) = arg.get("action_stake") {
                let promise_index = action["promise_index"].as_i64().unwrap() as u64;
                let amount = action["amount"].as_i64().unwrap() as u128;
                let public_key = from_base64(action["public_key"].as_str().unwrap());
                promise_batch_action_stake(
                    promise_index,
                    &amount as *const u128 as *const u64 as u64,
                    public_key.len() as u64,
                    public_key.as_ptr() as u64,
                );
                promise_index
            } else if let Some(action) = arg.get("action_add_key_with_full_access") {
                let promise_index = action["promise_index"].as_i64().unwrap() as u64;
                let public_key = from_base64(action["public_key"].as_str().unwrap());
                let nonce = action["nonce"].as_i64().unwrap() as u64;
                promise_batch_action_add_key_with_full_access(
                    promise_index,
                    public_key.len() as u64,
                    public_key.as_ptr() as u64,
                    nonce,
                );
                promise_index
            } else if let Some(action) = arg.get("action_add_key_with_function_call") {
                let promise_index = action["promise_index"].as_i64().unwrap() as u64;
                let public_key = from_base64(action["public_key"].as_str().unwrap());
                let nonce = action["nonce"].as_i64().unwrap() as u64;
                let allowance = action["allowance"].as_i64().unwrap() as u128;
                let receiver_id = action["receiver_id"].as_str().unwrap().as_bytes();
                let method_names = action["method_names"].as_str().unwrap().as_bytes();
                promise_batch_action_add_key_with_function_call(
                    promise_index,
                    public_key.len() as u64,
                    public_key.as_ptr() as u64,
                    nonce,
                    &allowance as *const u128 as *const u64 as u64,
                    receiver_id.len() as u64,
                    receiver_id.as_ptr() as u64,
                    method_names.len() as u64,
                    method_names.as_ptr() as u64,
                );
                promise_index
            } else if let Some(action) = arg.get("action_delete_key") {
                let promise_index = action["promise_index"].as_i64().unwrap() as u64;
                let public_key = from_base64(action["public_key"].as_str().unwrap());
                promise_batch_action_delete_key(
                    promise_index,
                    public_key.len() as u64,
                    public_key.as_ptr() as u64,
                );
                promise_index
            } else if let Some(action) = arg.get("action_delete_account") {
                let promise_index = action["promise_index"].as_i64().unwrap() as u64;
                let beneficiary_id = action["beneficiary_id"].as_str().unwrap().as_bytes();
                promise_batch_action_delete_account(
                    promise_index,
                    beneficiary_id.len() as u64,
                    beneficiary_id.as_ptr() as u64,
                );
                promise_index
            } else {
                unimplemented!()
            };
            let expected_id = arg["id"].as_i64().unwrap() as u64;
            assert_eq!(actual_id, expected_id);
            if let Some(ret) = arg.get("return") {
                if ret.as_bool().unwrap() == true {
                    promise_return(actual_id);
                }
            }
        }
    }
}
