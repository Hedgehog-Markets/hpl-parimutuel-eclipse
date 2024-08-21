import type { UmiPlugin } from "@metaplex-foundation/umi";

import { createHplParimutuelProgram } from "./generated";

export const hplParimutuel = (): UmiPlugin => ({
  install(umi) {
    umi.programs.add(createHplParimutuelProgram(), false);
  },
});
