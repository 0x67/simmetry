-- CreateEnum
CREATE TYPE "GameType" AS ENUM ('F12024', 'F12023', 'F12022', 'FH5', 'FH4', 'FM7', 'FM8');

-- CreateTable
CREATE TABLE "f1_data" (
    "id" TEXT NOT NULL,
    "game_type" "GameType" NOT NULL,
    "is_race_on" BOOLEAN NOT NULL DEFAULT false,
    "date_time" TIMESTAMPTZ NOT NULL,

    CONSTRAINT "f1_data_pkey" PRIMARY KEY ("id")
);
