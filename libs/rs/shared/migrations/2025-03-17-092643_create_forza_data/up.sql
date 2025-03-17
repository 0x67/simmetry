DO $$ BEGIN
    CREATE TYPE "ForzaType" AS ENUM ('FH5', 'FH4', 'FM7', 'FM8');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- CreateTable
CREATE TABLE IF NOT EXISTS "forza_data" (
    "id" TEXT NOT NULL,
    "game_type" "ForzaType" NOT NULL,
    "is_race_on" BOOLEAN NOT NULL DEFAULT false,
    "date_time" TIMESTAMPTZ NOT NULL,
    CONSTRAINT "forza_data_pkey" PRIMARY KEY ("id")
);
