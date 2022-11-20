// #[cfg(test)]
// mod tests {

//     // Note this useful idiom: importing names from outer (for mod tests) scope.

//     #[test]
//     fn test_decode_sprite() {
//         let mut target = 0xe0;
//         let mut result: [Bit; 8] = [false; 8];

//         for i in 0..8 {
//             let mask_result = target & 0x1;
//             target = target >> 1;

//             if mask_result == 1 {
//                 result[7 - i] = true;
//             } else {
//                 result[7 - i] = false;
//             }
//         }

//         assert_eq!(
//             result,
//             [true, true, true, false, false, false, false, false]
//         );
//     }
// }
