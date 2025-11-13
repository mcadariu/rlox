// fn compile() {
//     let line = -1;
//     loop {
//         let token: Token = scanToken();
//         match token.line {
//             line => {
//                 print!("%4d ", token.line);
//                 line = token.line;
//             }
//             _ => println!("   | ");
//         }
//         printf("%2d '%.*s'\n", token.type , token.length, token.start);
//
//         if (token.type == TOKEN_EOF) break;
//     }
// }