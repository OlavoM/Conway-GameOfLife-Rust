use std::io; 

const TAM: usize = 15;

fn clear_screen() {
    std::process::Command::new("sh").arg("-c").arg("clear").status().unwrap();
}

fn imprimir_matriz(matriz: &[[bool; TAM]; TAM]) {
    println!("  _0|_1|_2|_3|_4|_5|_6|_7|_8|_9|10|11|12|13|14|");
    for i in 0..TAM {
        print!("{} ", (i as u8 + 65) as char); // Letras das linhas

        for j in 0..TAM {
            if matriz[i][j] {
                print!("\u{2588}\u{2588}"); // Vivo imprime como quadrado preenchido
            } else {
                print!("__"); // Morto imprime como underline (quadrado vazio)
            }
            print!("|");
        }
        println!();
    }
}

fn posicao_existe(x: i32, y: i32) -> bool {
    if (x >= 0) && (x < TAM as i32) && (y >= 0) && (y < TAM as i32) {
        true
    } else {
        false
    }
}

fn contar_vizinhos(matriz: &[[bool; TAM]; TAM], linha: i32, coluna: i32) -> i32 {
    let mut contador = 0;
    if posicao_existe(linha - 1, coluna - 1) && matriz[(linha - 1) as usize][(coluna - 1) as usize] {
        contador += 1;
    }
    if posicao_existe(linha- 1, coluna) && matriz[linha - 1][coluna] {
        contador += 1;
    }
    if posicao_existe(linha - 1, coluna + 1) && matriz[linha - 1][coluna + 1] {
        contador += 1;
    }
    if posicao_existe(linha, coluna - 1) && matriz[linha][coluna - 1] {
        contador += 1;
    }
    if posicao_existe(linha, coluna + 1) && matriz[linha][coluna + 1] {
        contador += 1;
    }
    if posicao_existe(linha + 1, coluna - 1) && matriz[linha + 1][coluna - 1] {
        contador += 1;
    }
    if posicao_existe(linha + 1, coluna) && matriz[linha + 1][coluna] {
        contador += 1;
    }
    if posicao_existe(linha + 1, coluna + 1) && matriz[linha + 1][coluna + 1] {
        contador += 1;
    }
    contador
}

fn copiar_matriz(original: &[[bool; TAM]; TAM], copia: &mut [[bool; TAM]; TAM]) {
    for i in 0..TAM {
        for j in 0..TAM {
            copia[i][j] = original[i][j];
        }
    }
}

// Aplica as regras do jogo da vida, usando a matriz atual como base e escrevendo o proximo estado na matriz futura
fn executar_passo(atual: &[[bool; TAM]; TAM], futura: &mut [[bool; TAM]; TAM]) {
    for i in 0..TAM {
        for j in 0..TAM {
            let n_vizinhos = contar_vizinhos(atual, i, j);
            if atual[i][j] && (n_vizinhos == 2 || n_vizinhos == 3) {
                futura[i][j] = true;
            } else {
                futura[i][j] = false;
            }
        }
    }
}

fn execucao_na_tela(matriz_atual: &mut [[bool; TAM]; TAM], matriz_futura: &mut [[bool; TAM]; TAM]) {
    loop {
        clear_screen();
        imprimir_matriz(&matriz_atual);
        executar_passo(&mut matriz_atual, &mut matriz_futura); // Executa o passo na matriz futura
        copiar_matriz(&mut matriz_futura, &mut matriz_atual); // Atualiza a matriz

        println!("\n\nAperte Espaco para sair ou Enter para executar o próximo passo");

        let stdin = io::stdin();
        let mut handle = stdin.lock();
        let mut buf = String::new();
        handle.read_line(&mut buf).expect("Falha ao ler a entrada");

        if buf.trim() == "" {
            break;
        }
    }
}


fn submenu_preenchimento_manual(matriz_atual: &mut [[bool; TAM]; TAM], matriz_futura: &mut [[bool; TAM]; TAM]) {
    loop {
        clear_screen();
        imprimir_matriz(&matriz_atual);
        println!("\n\n\nEscolha uma opcao:");
        println!("0 - Voltar para o menu principal");
        println!("1 - Preencher uma celula");
        println!("2 - executar");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Falha ao ler a opção");
        let opcao: i32 = opcao.trim().parse().expect("Entrada inválida");

        if opcao == 0 {
            break;
        } else if opcao == 1 {
            let mut caractere_y = String::new();
            let mut pos_y: i32;
            let mut pos_x: i32;

            pos_y = -1;
            while pos_y < 0 || pos_y > 14 {
                println!("Digite a coordenada Y (em letra maiúscula):");
                io::stdin().read_line(&mut caractere_y).expect("Falha ao ler a coordenada Y");
                let caractere_y: char = caractere_y.trim().chars().next().expect("Entrada inválida");

                pos_y = (caractere_y as i32) - 65; // Converte a letra para um índice inteiro
            }

            let mut pos_x_input = String::new();
            pos_x = -1;
            while pos_x < 0 || pos_x > 14 {
                println!("Digite a coordenada X:");
                io::stdin().read_line(&mut pos_x_input).expect("Falha ao ler a coordenada X");
                pos_x = pos_x_input.trim().parse().expect("Entrada inválida");
            }

            matriz_atual[pos_y as usize][pos_x as usize] = true; // Define a célula especificada como vivo

        } else if opcao == 1 {
            execucao_na_tela(&mut matriz_atual, &mut matriz_futura);
            break;
        }
    }
}

fn main() {
    let mut matriz_atual: [[bool; TAM]; TAM] = [[false; TAM]; TAM];
    let mut matriz_futura: [[bool; TAM]; TAM] = [[false; TAM]; TAM];

    matriz_atual[1][1] = true;

    loop {
        clear_screen();
        println!("Jogo da vida de Conway\n");
        imprimir_matriz(&matriz_atual);
        println!("\n\n\nEscolha uma opcao:");
        println!("0 - Sair");
        println!("1 - Preencher manualmente a matriz");
        println!("2 - Usar um padrao pronto");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Falha ao ler a opção");
        let opcao: i32 = opcao.trim().parse().expect("Entrada inválida");

        if opcao == 0 {
            break;
        } else if opcao == 1 {
            submenu_preenchimento_manual(&mut matriz_atual, &mut matriz_futura);
        }


    }
}

