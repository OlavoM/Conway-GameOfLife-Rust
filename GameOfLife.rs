use std::io; 

const TAM: usize = 15;

fn clear_screen() {
    std::process::Command::new("sh").arg("-c").arg("clear").status().unwrap();
}

fn imprimir_matriz(matriz: &[[bool; TAM]; TAM]) {
    let mut matrizStringBuffer = String::new();
    matrizStringBuffer += "  _0|_1|_2|_3|_4|_5|_6|_7|_8|_9|10|11|12|13|14|\n";
    for i in 0..TAM {
        matrizStringBuffer += ("{} ", (i as u8 + 65) as char); // Letras das linhas

        for j in 0..TAM {
            if matriz[i][j] {
                matrizStringBuffer += "\u{2588}\u{2588}"; // Vivo imprime como quadrado preenchido
            } else {
                matrizStringBuffer += "__"; // Morto imprime como underline (quadrado vazio)
            }
            matrizStringBuffer += "|";
        }

        matrizStringBuffer += "\n";
    }
    print!(matrizStringBuffer);
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
    if posicao_existe(linha- 1, coluna) && matriz[(linha - 1) as usize][coluna as usize] {
        contador += 1;
    }
    if posicao_existe(linha - 1, coluna + 1) && matriz[(linha - 1) as usize][(coluna + 1) as usize] {
        contador += 1;
    }
    if posicao_existe(linha, coluna - 1) && matriz[linha as usize][(coluna - 1) as usize] {
        contador += 1;
    }
    if posicao_existe(linha, coluna + 1) && matriz[linha as usize][(coluna + 1) as usize] {
        contador += 1;
    }
    if posicao_existe(linha + 1, coluna - 1) && matriz[(linha + 1) as usize][(coluna - 1) as usize] {
        contador += 1;
    }
    if posicao_existe(linha + 1, coluna) && matriz[(linha + 1) as usize][coluna as usize] {
        contador += 1;
    }
    if posicao_existe(linha + 1, coluna + 1) && matriz[(linha + 1) as usize][(coluna + 1) as usize] {
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

fn padrao_block(matriz: &mut [[bool; TAM]; TAM]) {
    matriz[6][6] = true;
    matriz[6][7] = true;
    matriz[7][6] = true;
    matriz[7][7] = true;
}

fn padrao_bee_hive(matriz: &mut [[bool; TAM]; TAM]) {
    matriz[5][6] = true;
    matriz[5][7] = true;
    matriz[6][5] = true;
    matriz[6][8] = true;
    matriz[7][6] = true;
    matriz[7][7] = true;
}

fn padrao_blinker(matriz: &mut [[bool; TAM]; TAM]) {
    matriz[5][6] = true;
    matriz[6][6] = true;
    matriz[7][6] = true;
}

fn padrao_pulsar(matriz: &mut [[bool; TAM]; TAM]) {
    matriz[1][3] = true; matriz[1][4] = true; matriz[1][5] = true;
    matriz[1][9] = true; matriz[1][10] = true; matriz[1][11] = true;
    matriz[3][1] = true; matriz[3][6] = true; matriz[3][8] = true;
    matriz[3][13] = true; matriz[4][1] = true; matriz[4][6] = true;
    matriz[4][8] = true; matriz[4][13] = true; matriz[5][1] = true;
    matriz[5][6] = true; matriz[5][8] = true; matriz[5][13] = true;
    matriz[6][3] = true; matriz[6][4] = true; matriz[6][5] = true;
    matriz[6][9] = true; matriz[6][10] = true; matriz[6][11] = true;
    matriz[8][3] = true; matriz[8][4] = true; matriz[8][5] = true;
    matriz[8][9] = true; matriz[8][10] = true; matriz[8][11] = true;
    matriz[9][1] = true; matriz[9][6] = true; matriz[9][8] = true;
    matriz[9][13] = true; matriz[10][1] = true; matriz[10][6] = true;
    matriz[10][8] = true; matriz[10][13] = true; matriz[13][3] = true;
    matriz[13][4] = true; matriz[13][5] = true; matriz[13][9] = true;
    matriz[13][10] = true; matriz[13][11] = true; matriz[11][1] = true;
    matriz[11][6] = true; matriz[11][8] = true; matriz[11][13] = true;
}

fn padrao_glider(matriz: &mut [[bool; TAM]; TAM]) {
    matriz[1][2] = true;
    matriz[2][3] = true;
    matriz[3][1] = true;
    matriz[3][2] = true;
    matriz[3][3] = true;
}

fn padrao_heavy_weight_spaceship(matriz: &mut [[bool; TAM]; TAM]) {
    matriz[5][2] = true; matriz[5][3] = true;
    matriz[5][4] = true; matriz[5][5] = true;
    matriz[5][6] = true; matriz[5][7] = true;
    matriz[6][1] = true; matriz[6][7] = true;
    matriz[7][7] = true; matriz[8][1] = true;
    matriz[8][6] = true; matriz[9][3] = true;
    matriz[9][4] = true;
}


// Aplica as regras do jogo da vida, usando a matriz atual como base e escrevendo o proximo estado na matriz futura
fn executar_passo(atual: &[[bool; TAM]; TAM], futura: &mut [[bool; TAM]; TAM]) {
    for i in 0..TAM {
        for j in 0..TAM {
            let n_vizinhos = contar_vizinhos(atual, i as i32, j as i32);
            if atual[i][j] && (n_vizinhos == 2 || n_vizinhos == 3) {
                futura[i][j] = true;
            } else if !atual[i][j] && n_vizinhos == 3 {
                futura[i][j] = true;
            } else {
                futura[i][j] = false;
            }
        }
    }
}

fn execucao_na_tela(mut matriz_atual: [[bool; TAM]; TAM], mut matriz_futura: [[bool; TAM]; TAM]) {
    loop {
        clear_screen();
        imprimir_matriz(&matriz_atual);
        executar_passo(&mut matriz_atual, &mut matriz_futura); // Executa o passo na matriz futura
        copiar_matriz(&mut matriz_futura, &mut matriz_atual); // Atualiza a matriz

        println!("\n\nDigite 0 para sair ou Enter para executar o próximo passo");

        let stdin = io::stdin();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Falha ao ler a entrada");

        if buf.trim() == "0" {
            break;
        }
    }
}


fn submenu_preenchimento_manual(mut matriz_atual: [[bool; TAM]; TAM], mut matriz_futura: [[bool; TAM]; TAM]) {
    loop {
        clear_screen();
        imprimir_matriz(&matriz_atual);
        println!("\n\n\nEscolha uma opção:");
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

        } else if opcao == 2 {
            execucao_na_tela(matriz_atual, matriz_futura);
            break;
        }
    }
}

fn submenu_padrao_pronto(mut matriz_atual: [[bool; TAM]; TAM], mut matriz_futura: [[bool; TAM]; TAM]) {
    loop {
        clear_screen();
        imprimir_matriz(&matriz_atual);
        println!("\n\n\nEscolha uma opcao:");
        println!("0 - Voltar para o menu principal");
        println!("1 - Block");
        println!("2 - Bee-hive");
        println!("3 - Blinker");
        println!("4 - Pulsar");
        println!("5 - Glider");
        println!("6 - Heavy-weight spaceship");
        println!("7 - Executar");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Falha ao ler a opção");
        let opcao: i32 = opcao.trim().parse().expect("Entrada inválida");

        if opcao == 0 {
            break;
        } else if opcao == 1 { // Preenche a matriz com o padrão escolhido, no caso o Block
            padrao_block(&mut matriz_atual);
        } else if opcao == 2 {
            padrao_bee_hive(&mut matriz_atual);
        } else if opcao == 3 {
            padrao_blinker(&mut matriz_atual);
        } else if opcao == 4 {
            padrao_pulsar(&mut matriz_atual);
        } else if opcao == 5 {
            padrao_glider(&mut matriz_atual);
        } else if opcao == 6 {
            padrao_heavy_weight_spaceship(&mut matriz_atual);
        } else if opcao == 7 {
            execucao_na_tela(matriz_atual, matriz_futura);
            break;
        }
    }
}

fn main() {
    let mut matriz_atual: [[bool; TAM]; TAM] = [[false; TAM]; TAM];
    let mut matriz_futura: [[bool; TAM]; TAM] = [[false; TAM]; TAM];

    loop {
        clear_screen();
        println!("Jogo da vida de Conway\n");
        imprimir_matriz(&matriz_atual);
        println!("\n\n\nEscolha uma opção:");
        println!("0 - Sair");
        println!("1 - Preencher manualmente a matriz");
        println!("2 - Usar um padrao pronto");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Falha ao ler a opção");
        let opcao: i32 = opcao.trim().parse().expect("Entrada inválida");

        if opcao == 0 {
            break;
        } else if opcao == 1 {
            submenu_preenchimento_manual(matriz_atual, matriz_futura);
        } else if opcao == 2 {
            submenu_padrao_pronto(matriz_atual, matriz_futura);
        }


    }
}

