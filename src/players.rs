
/// Const-definable player struct used to define the list of champs someone is willing to play. 
#[derive(Copy, Clone, Debug)]
pub struct Player {
    /// Name of the player
    pub name: &'static str,
    /// List of champs that player would be willing to play. 
    pub champs: &'static [&'static str],
}

/// List of people playing
pub const PLAYERS: &'static [Player] = &[VENUS, EMMA];

const MADDIE: Player = Player {
    name: "Maddie",
    champs: &[
        "Caitlyn",
        "Jinx",
        "Ashe",
        "Jhin"
    ]
};

const TONI: Player = Player {
    name: "Toni",
    champs: &[
        "Vel'Koz",
        "Evelynn",
        "Cho'Gath",
        "Briar",
        "Morgana",
        "Kindred"
    ]
};

const VENUS: Player = Player {
    name: "Venus",
    champs: &[
        "Mordekaiser",
        "Blitzcrank",
        "Lux",
        "Pantheon",
        "Illaoi",
        "Gangplank"
    ],
};

const EMMA: Player = Player {
    name: "Emma",
    champs: &[
        "Diana",
        "Pyke",
        "Akali",
        "Fizz",
        "Ahri",
        "Jinx",
        "Kalista",
        "LeBlanc",
        "Lux",
        "Gwen",
        "Ezreal",
        "Soraka",
        "Renata Glasc",
        "Yuumi",
        "Seraphine",
        "Kindred",
        "Irelia",
        "Azir",
        "Kai'Sa",
        "Karma",
        "Kennen",
        "Mordekaiser",
        "Nami",
        "Quinn",
        "Senna",
        "Sivir",
        "Shyvana",
        "Taliyah",
        "Varus",
        "Viego",
        "Xayah",
        "Vayne"
    ],
};
