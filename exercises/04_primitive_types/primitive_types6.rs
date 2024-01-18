// primitive_types6.rs
//
// Use a tuple index to access the second element of `numbers`. You can put the
// expression for the second element where ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand
// for a hint.

// Au vu de l'erreur, nous devions juste remplacer les point d'interrogation par le chiffre 2 car le nombre est 2.

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = 2;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
