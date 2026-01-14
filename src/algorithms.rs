use crate::knapsack::{Item, Knapsack};
use crate::table::init_table;

pub fn dynamic_programming(items: &Vec<Item>, capacity: u16) -> Knapsack {
    let rows = items.len();
    let columns = capacity as usize;

    // Inicializa a tabela de PD com dimensões (itens + 1) x (capacidade + 1)
    // Linha/coluna extra para caso base (0 itens / 0 capacidade = 0 valor)
    let mut table = init_table(rows + 1, columns + 1);

    // Preenche a tabela de PD
    // Começa na linha 1 (linha 0 é toda zeros - caso base de nenhum item)
    for row_index in 1..=rows {
        // Considera cada capacidade possível de 1 até o máximo
        for column_index in 1..=columns {
            let current_item = &items[row_index - 1];
            let item_weight = current_item.weight as usize;
            let item_value = current_item.value;

            // Verifica se o item atual cabe na capacidade atual
            if item_weight <= column_index {
                // Opção 1: Não pegar o item atual
                // Valor = melhor valor dos itens anteriores na mesma capacidade
                let first = table[row_index - 1][column_index];

                // Opção 2: Pegar o item atual
                // Valor = melhor valor dos itens anteriores na (capacidade restante) + valor do item atual
                // Capacidade restante = capacidade atual - peso do item atual
                let second = table[row_index - 1][column_index - item_weight] + item_value;

                // Escolhe a opção que dá o valor máximo
                let first_or_second_max_value = first.max(second);

                table[row_index][column_index] = first_or_second_max_value;
            } else {
                // Item não cabe - copia valor da linha anterior (sem este item)
                table[row_index][column_index] = table[row_index - 1][column_index];
            }
        }
    }

    // Retrocede pela tabela para encontrar quais itens foram selecionados
    let mut knapsack = Knapsack::new(capacity);
    let mut row = rows; // Começa no último item
    let mut col = columns; // Começa na capacidade total

    while row > 0 && col > 0 {
        // Se o valor mudou da linha anterior, este item foi incluído
        if table[row][col] != table[row - 1][col] {
            let current_item = &items[row - 1];
            knapsack.add_item(current_item.clone());

            // Reduz a capacidade restante pelo peso deste item
            col -= current_item.weight as usize;
        }

        // Move para o item anterior
        row -= 1;
    }

    knapsack
}
