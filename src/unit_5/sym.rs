
#[repr(C)]
pub struct SymbolTable {
    start: *const u8,
    end: *const u8,
}

impl SymbolTable {
    pub const fn new() -> Self {
        unsafe extern "C" {
            static symbol_table: u8;
            static symbol_table_end: u8;
        }

        unsafe {
            Self {
                start: &symbol_table as *const u8,
                end: &symbol_table_end as *const u8,
            }
        }
    }

    pub fn find(&self, name: &str) -> Option<u64> {
        unsafe {
            let mut current = self.start;

            while current < self.end {
                let address = *(current as *const u64);

                if address == 0 {
                    break;
                }

                // 比较字符串
                let name_ptr = current.add(8);
                if self.str_eq(name_ptr, name.as_bytes()) {
                    return Some(address);
                }

                // 移动到下一个条目
                let name_len = self.strlen(name_ptr) + 1;
                let total_len = 8 + name_len;
                let aligned_len = (total_len + 7) & !7;
                current = current.add(aligned_len);
            }
        }

        None
    }

    unsafe fn strlen(&self, s: *const u8) -> usize {
        let mut len = 0;
        while *s.add(len) != 0 {
            len += 1;
        }
        len
    }

    unsafe fn str_eq(&self, ptr: *const u8, target: &[u8]) -> bool {
        for (i, &byte) in target.iter().enumerate() {
            if *ptr.add(i) != byte {
                return false;
            }
        }
        *ptr.add(target.len()) == 0
    }
}
