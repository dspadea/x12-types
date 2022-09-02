use crate::v004010::Transmission;

pub fn is_equal_payload<T: PartialEq>(src: &Transmission<T>, target: &Transmission<T>) -> bool{
    let src_group = &src.functional_group;
    for src_item in src_group {
        let x = src_item.eq(target.functional_group.get(0).unwrap());
        if x== false{
            return false;
        }
    }
    true
}