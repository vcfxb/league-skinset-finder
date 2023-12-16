/// Const-definable player struct used to define the list of champs someone is willing to play.
#[derive(Copy, Clone, Debug)]
pub struct Player {
    /// Name of the player
    pub name: &'static str,
    /// List of champs that player would be willing to play.
    pub champs: &'static [&'static str],
}

/// Skinsets we're not playing for various reasons.
const SKINSET_BLACKLIST: &'static [&'static str] = &[
    // Blacklisted for being aesthetically incoherent
    "Legacy",
    // Blacklisted for being ugly.
    "Battlecast",
    // Many ugly skins
    "Infernal",
];

// Taken from other file getting removed.
// /// Used to manually add lanes to champions that are not otherwise considered kosher.
// const MANUAL_LANE_OVERRIDES: &'static [(&'static str, Lane)] = &[
//     ("Caitlyn", Lane::Top),
//     ("Cho'Gath", Lane::Support),
// ];

/// List of people playing
pub const PLAYERS: &'static [Player] = &[TONI, VENUS, EMMA];

const MADDIE: Player = Player {
    name: "Maddie",
    champs: &["Caitlyn", "Jinx", "Ahri", "Ashe"],
};

const TONI: Player = Player {
    name: "Toni",
    champs: &[
        "Vel'Koz", "Evelynn", "Cho'Gath", "Briar", "Morgana", "Kindred",
    ],
};

const VENUS: Player = Player {
    name: "Venus",
    champs: &[
        "Mordekaiser",
        "Blitzcrank",
        "Lux",
        "Pantheon",
        "Illaoi",
        "Gangplank",
        // "Seraphine",
        "Rakan",
        "Nasus",
        "Garen",
        "Karthus",
        "Hecarim",
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
        "Vayne",
        "Aphelios",
        "Trundle",
        "Aatrox",
        "Rakan",
    ],
};
