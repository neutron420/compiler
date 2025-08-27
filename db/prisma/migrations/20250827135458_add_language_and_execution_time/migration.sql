/*
  Warnings:

  - You are about to drop the column `executionTime` on the `executions` table. All the data in the column will be lost.
  - You are about to drop the column `memoryUsage` on the `executions` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "public"."executions" DROP COLUMN "executionTime",
DROP COLUMN "memoryUsage",
ADD COLUMN     "execution_time_ms" INTEGER,
ADD COLUMN     "language" TEXT NOT NULL DEFAULT 'custom',
ADD COLUMN     "memory_usage" INTEGER;
