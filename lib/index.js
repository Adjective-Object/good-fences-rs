let native = require("../native/index.node");
let path = require("path");
let fs = require("fs");

module.exports = {
  run(options) {
    let root_dir = options.rootDir ? options.rootDir : process.cwd();
    let project_file_path = path.join(
      root_dir,
      options.project || "tsconfig.json"
    );
    let dir_listing = fs.readdirSync(root_dir, {
      withFileTypes: true,
    });
    let folder_paths = dir_listing
      .filter(
        (file) =>
          file.isDirectory() &&
          (options.ignoreExternalFences == false ||
            file.name !== "node_modules")
      )
      .map((file) => path.join(root_dir, file.name));
    console.log(root_dir, project_file_path, folder_paths);
    return native.run_good_fences(project_file_path, folder_paths);
  },
};
