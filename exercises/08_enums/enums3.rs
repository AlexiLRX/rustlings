// enums3.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a
// hint.
// Premiere étape j'enumere tous les types de messages differents, je me base sur chacune des erreurs et pour le type je me fie
// au code ou il y a ecrit si c'est un string ou un u8 ou un i32 etc.
// Ensuite je suis aller récuperer dans le enums2 la partie "message" puis j'ai vu quand dans les commentaire il fallait utiliser 
// "a match expression" j'ai donc cherché sur internet comment ca s'utilisais puis il y avait encore une erreur au niveau de la ligne 15
// J'ai donc vu dans les commentaire qu'on devait mettre 2 paranthése pour ce cas la. J'ai donc mis deux paranthése la ou
// les erreurs étaient identifiés.

enum Message {
    // TODO: implement the message variant types based on their usage below
    ChangeColor((u8, u8, u8)),
    Echo(String),
    Move(Point),
    Quit,
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) {
        self.message = s
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message variants
        // Remember: When passing a tuple as a function argument, you'll need extra parentheses:
        // fn function((t, u, p, l, e))
        match message {
        Message::ChangeColor(random) => self.change_color(random),
        Message::Echo(a) => self.echo(a),
        Message::Move(b) => self.move_position(b),
        Message::Quit => self.quit(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        state.process(Message::ChangeColor((255, 0, 255)));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "Hello world!");
    }
}
