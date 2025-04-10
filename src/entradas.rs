use std::iter::Peekable;
use std::str::Chars;

// Separa el input en tokens según las reglas:
// - Si un token inicia con `."` se consume hasta la comilla de cierre (incluyendo el punto y ambas comillas)
// - De lo contrario, se acumula hasta el siguiente espacio.
pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }
        if c == '.' {
            tokens.push(tokenize_special(&mut chars));
        } else {
            tokens.push(tokenize_normal(&mut chars));
        }
    }
    tokens
}

// Procesa tokens que comienzan con '.'.
// Si le sigue una comilla, captura todo hasta la siguiente comilla incluyendo
// el símbolo de apertura y cierre. Si no, acumula el resto como token normal.
fn tokenize_special(chars: &mut Peekable<Chars>) -> String {
    let mut token: String = String::new();
    if let Some(dot) = chars.next() {
        token.push(dot);
    } else {
        return token;
    }
    if let Some(&c) = chars.peek() {
        if c == '"' {
            if let Some(q) = chars.next() {
                token.push(q);
            } else {
                return token;
            }
            while let Some(ch) = chars.next() {
                token.push(ch);
                if ch == '"' {
                    break;
                }
            }
        } else {
            token.push_str(&tokenize_normal(chars));
        }
    }
    token
}

// Acumula caracteres en un token normal hasta el próximo espacio.
fn tokenize_normal(chars: &mut Peekable<Chars>) -> String {
    let mut token: String = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            break;
        }
        if let Some(ch) = chars.next() {
            token.push(ch);
        }
    }
    token
}