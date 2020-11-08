import { run } from "../index";
import { join } from "path";

describe("runner", () => {
  it("returns the expected results", () => {
    const actualResults = run({
      rootDir: join(__dirname, "..", "..", "sample"),
    });

    expect(actualResults).toMatchInlineSnapshot();
  });
});
