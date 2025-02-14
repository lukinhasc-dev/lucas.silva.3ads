# lucas.silva.3ads

Claro, Lukinhas! Aqui está o texto do desafio formatado para o **"Sobre Mim"** do GitHub, estruturado de forma organizada e estilizada usando **Markdown**.  

---  

## 💡 Desafio do Dia: Inversão de String  

### 📌 **Passo 1: Introdução do Desafio**  
O desafio é criar uma função em **Rust** que inverta uma string.  
A função deve receber uma string como entrada e retornar a string invertida.  

#### 🔹 **Exemplo:**  
✅ **Entrada:** `"Data Structures"`  
✅ **Saída:** `"serutcurtS ataD"`  

---  

### 🛠 **Passo 2: Configurar o Projeto com Cargo**  
Crie um novo projeto Rust como uma biblioteca:  
```sh
cargo new reverse_string --lib
```
➡️ **O `--lib` cria um projeto como biblioteca, ideal para testes unitários.**  

Entre no diretório do projeto:  
```sh
cd reverse_string
```

#### 📂 **Estrutura inicial do projeto:**
```
reverse_string/
├── Cargo.toml
└── src/
    └── lib.rs
```
📌 **Explicação:**  
- `Cargo.toml`: contém as configurações do projeto.  
- `src/lib.rs`: onde a lógica da biblioteca será implementada.  
- **Adicione `Cargo.lock` ao `.gitignore` para evitar conflitos no Git** (o `/target` já está ignorado).  

---  

### 🔄 **Passo 3: Implementar a Função**  
Abra o arquivo **`src/lib.rs`** e implemente:  

✅ **Requisitos:**  
- Criar uma função pública chamada `reverse`.  
- A função deve receber uma string como entrada (`&str`) e retornar uma nova string invertida.  
- Utilize métodos como `.chars()`, `.rev()`, `.collect()`.  
- A função deve ser exportada corretamente para ser usada em outros módulos ou testes.  

📌 **Dica:** Consulte a documentação oficial do Rust para mais detalhes!  

---  

### 📁 **Passo 4: Criar Testes na Pasta `tests`**  
Crie uma pasta chamada **`tests`** no diretório raiz do projeto:  
```sh
mkdir tests
```
Dentro da pasta **`tests`**, crie um arquivo para os testes de integração:  
```sh
touch tests/integration_test.rs
```

🔹 **No arquivo `integration_test.rs`, escreva testes que:**  
- Importem a biblioteca com `use reverse_string;`.  
- Testem diferentes cenários, como:  
  - Strings normais (**ex.:** `"Data Structures"`).  
  - Strings vazias (**ex.:** `""`).  
  - Strings com um único caractere (**ex.:** `"A"`).  
- Utilizem `assert_eq!` para validar os resultados.  

---  

### ✅ **Passo 5: Executar os Testes**  
Para rodar os testes, use o comando:  
```sh
cargo test
```
📌 Isso garante que a implementação está funcionando corretamente!  

---

🎯 **Desafio concluído?** Compartilhe seu código e me marque! 😎🚀  

---
