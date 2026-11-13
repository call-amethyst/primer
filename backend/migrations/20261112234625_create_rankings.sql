CREATE TYPE agent AS ENUM (
    'ASTRA',
    'BREACH',
    'BRIMSTONE',
    'CHAMBER',
    'CLOVE',
    'CYPHER',
    'DEADLOCK',
    'FADE',
    'GEKKO',
    'HARBOR',
    'HUSH',
    'ISO',
    'JETT',
    'KAYO',
    'KILLJOY',
    'MIKS',
    'NEON',
    'OMEN',
    'PHOENIX',
    'RAZE',
    'REYNA',
    'SAGE',
    'SKYE',
    'SOVA',
    'TEJO',
    'VETO',
    'VIPER',
    'VYSE',
    'WAYLAY',
    'YORU'
);

CREATE TYPE game_map AS ENUM (
    'BIND',
    'HAVEN',
    'SPLIT',
    'ASCENT',
    'ICEBOX',
    'BREEZE',
    'FRACTURE',
    'PEARL',
    'LOTUS',
    'SUNSET',
    'ABYSS',
    'CORRODE',
    'SUMMIT'
);

CREATE TYPE agent_rank AS ENUM (
    'MAIN',
    'GOOD',
    'AVERAGE',
    'BAD',
    'NOTPLAYED'
);

CREATE TABLE rankings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    user_id UUID NOT NULL REFERENCES users(id)
        ON DELETE CASCADE,

    map game_map NOT NULL,
    agent agent NOT NULL,
    rank agent_rank NOT NULL,

    UNIQUE (user_id, map, agent)
);
