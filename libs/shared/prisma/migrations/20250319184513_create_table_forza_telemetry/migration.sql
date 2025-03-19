-- CreateEnum
CREATE TYPE "forza_type" AS ENUM ('FH5', 'FH4', 'FM7', 'FM8');

-- CreateEnum
CREATE TYPE "forza_car_class" AS ENUM ('D', 'C', 'B', 'A', 'S1', 'S2', 'S3', 'X');

-- CreateEnum
CREATE TYPE "forza_drive_type" AS ENUM ('FWD', 'RWD', 'AWD');

-- CreateTable
CREATE TABLE "forza_telemetry" (
    "id" TEXT NOT NULL,
    "user_id" TEXT NOT NULL,
    "game_type" "forza_type" NOT NULL,
    "date_time" TIMESTAMPTZ NOT NULL,
    "is_race_on" BOOLEAN NOT NULL DEFAULT false,
    "drive_type" "forza_drive_type" NOT NULL,
    "car_class" "forza_car_class" NOT NULL,
    "car_id" INTEGER NOT NULL,
    "car_performance_index" INTEGER NOT NULL,
    "num_cylinders" INTEGER NOT NULL,
    "engine_max_rpm" REAL NOT NULL,
    "engine_idle_rpm" REAL NOT NULL,
    "engine_current_rpm" REAL NOT NULL,
    "track_id" INTEGER,
    "speed" REAL,
    "power" REAL,
    "torque" REAL,
    "boost" REAL,
    "fuel" REAL,
    "distance_traveled" REAL,
    "best_lap" REAL,
    "last_lap" REAL,
    "current_lap" REAL,
    "current_race_time" REAL,
    "lap_number" INTEGER,
    "race_position" INTEGER,
    "acceleration" INTEGER,
    "brake" INTEGER,
    "clutch" INTEGER,
    "handbrake" INTEGER,
    "gear" INTEGER,
    "steer" INTEGER,
    "normalized_driving_lane" INTEGER,
    "normalized_ai_brake_difference" INTEGER,
    "accelerations" JSONB NOT NULL,
    "velocities" JSONB NOT NULL,
    "angular_velocities" JSONB NOT NULL,
    "orientations" JSONB NOT NULL,
    "normalized_suspension_travels" JSONB NOT NULL,
    "tire_slips_ratios" JSONB NOT NULL,
    "wheel_rotation_speeds" JSONB NOT NULL,
    "wheel_on_rumble_strips" JSONB NOT NULL,
    "wheel_in_puddles" JSONB NOT NULL,
    "surface_rumbles" JSONB NOT NULL,
    "tire_slip_angles" JSONB NOT NULL,
    "tire_combined_slips" JSONB NOT NULL,
    "suspension_travel_meters" JSONB NOT NULL,
    "tire_temperatures" JSONB,
    "tire_wears" JSONB,
    "positions" JSONB,

    CONSTRAINT "forza_telemetry_pkey" PRIMARY KEY ("user_id","game_type","date_time")
);

-- AddForeignKey
ALTER TABLE "forza_telemetry" ADD CONSTRAINT "forza_telemetry_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
