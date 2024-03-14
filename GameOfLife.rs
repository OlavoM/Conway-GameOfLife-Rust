fn cls() {
    std::process::Command::new("sh").arg("-c").arg("clear").status().unwrap();
}

fn main() {
    let mut matriz_atual: [[bool; 5]; 5] = [[false; 5]; 5];
    let mut matriz_futura: [[bool; 5]; 5] = [[false; 5]; 5];

    loop {
        cls();
        println!("Jogo da vida de Conway\n");
        imprimirMatriz(matriz_atual);
        println!("\n\n\nEscolha uma opcao:");
        println!("0 - Sair");
        println!("1 - Preencher manualmente a matriz");
        println!("2 - Usar um padrao pronto");

        if (opcao == 0) {
            break;
        }

    }
}

