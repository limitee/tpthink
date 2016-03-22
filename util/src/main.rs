extern crate util;
use util::DigestUtil;

fn main() {
	assert!("900150983cd24fb0d6963f7d28e17f72" == DigestUtil::md5("abc"));
	assert!("a7bac2239fcdcb3a067903d8077c4a07" == DigestUtil::md5("中文"));
}
