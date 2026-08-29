CREATE TABLE "users" (
    "id" BLOB NOT NULL,
    "username" TEXT NOT NULL,
    "passkey" TEXT NOT NULL,
    PRIMARY KEY ("id")
);
-- #[toasty::breakpoint]
CREATE UNIQUE INDEX "index_users_by_username" ON "users" ("username");
-- #[toasty::breakpoint]
CREATE TABLE "book_progresses" (
    "document_id" TEXT NOT NULL,
    "username" TEXT NOT NULL,
    "percentage" REAL NOT NULL,
    "progress" TEXT NOT NULL,
    "device" TEXT NOT NULL,
    "created_at" TEXT NOT NULL,
    "updated_at" TEXT NOT NULL,
    PRIMARY KEY ("document_id")
);
-- #[toasty::breakpoint]
CREATE UNIQUE INDEX "index_book_progresses_by_document_id_and_username" ON "book_progresses" ("document_id", "username");
