use core::fmt::Debug;

use alloc::boxed::Box;
use alloc::string::String;

use crate::Validator;

#[inline]
pub fn attribute_incorrect_format(attribute_name: &str, correct_usage: &[&str]) -> ! {
    panic!(
        "You are using an incorrect format of the `{}` attribute.{}",
        attribute_name,
        concat_string_slice_array(correct_usage)
    )
}

#[inline]
pub fn parameter_incorrect_format(parameter_name: &str, correct_usage: &[&str]) -> ! {
    panic!(
        "You are using an incorrect format of the `{}` parameter.{}",
        parameter_name,
        concat_string_slice_array(correct_usage)
    )
}

#[inline]
pub fn derive_attribute_not_set_up_yet(attribute_name: &str) -> ! {
    panic!(
        "You are using `{}` in the `derive` attribute, but it has not been set up yet.",
        attribute_name
    )
}

#[inline]
pub fn reset_parameter(parameter_name: &str) -> ! {
    panic!("Try to reset the `{}` parameter.", parameter_name)
}

#[inline]
pub fn unknown_parameter(attribute_name: &str, parameter_name: &str) -> ! {
    panic!("Unknown parameter `{}` used in the `{}` attribute.", parameter_name, attribute_name)
}

#[inline]
pub fn validator_only_support_for_item(validator: Validator, item: Box<dyn Debug>) -> ! {
    panic!("This `{:?}` validator only support for {:?}.", validator, item)
}

// TODO patterns

#[inline]
pub fn validator_format_incorrect() -> ! {
    attribute_incorrect_format("validator", &[stringify!(#[validator(validator_name)])])
}

fn concat_string_slice_array(array: &[&str]) -> String {
    let len = array.len();

    if len == 0 {
        String::new()
    } else {
        let mut string = String::from(" It needs to be formed into ");

        let mut iter = array.iter();

        let first = iter.next().unwrap();

        string.push('`');
        string.push_str(&first.replace("\n", ""));
        string.push('`');

        if len > 2 {
            for s in iter.take(len - 2) {
                string.push_str(", `");
                string.push_str(&s.replace("\n", ""));
                string.push('`');
            }
        }

        if len > 1 {
            string.push_str(", or `");
            string.push_str(&array[len - 1].replace("\n", ""));
            string.push('`');
        }

        string.push('.');

        string
    }
}
