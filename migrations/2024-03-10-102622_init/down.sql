-- This file should undo anything in `up.sql`
DROP TABLE "_Likes";
DROP TABLE "_Boosts";
DROP TABLE "_Mentions";
DROP TABLE "_Followers";
DROP TABLE "Printer";
DROP TABLE "Note";
DROP TABLE "File";
DROP TABLE "Model";
DROP TABLE "Account";
DROP TABLE "Profile";
DROP TYPE "ModifiedScale";
DROP TYPE "EventAudience";
DROP EXTENSION pg_uuidv7;
