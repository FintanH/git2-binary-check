use git2::*;

fn main() {
    // Initial commit of this repo
    const INIT: &str = "373b3404371e359404392aff715d504293112c8b";

    // Commit containing bins
    const BINS: &str = "abdedcec89e3c48ac520e4a96c7ea39364316b9e";

    // Open this current repo
    let repo = Repository::open(".").unwrap();

    // Get the trees for the two commits
    let init = {
        let c = repo.find_commit(Oid::from_str(INIT).unwrap()).unwrap();
        c.tree().unwrap()
    };
    let bins = {
        let c = repo.find_commit(Oid::from_str(BINS).unwrap()).unwrap();
        c.tree().unwrap()
    };

    // Ensure we're not skipping the binary check
    let mut opts = DiffOptions::default();
    opts.skip_binary_check(false);

    // Obtain the diff and print the file names and if they're binary
    let diff = repo
        .diff_tree_to_tree(Some(&init), Some(&bins), Some(&mut opts))
        .unwrap();
    for delta in diff.deltas() {
        let file = delta.new_file();
        println!("Path: {:?}", file.path().unwrap());
        println!("Is binary: {}", file.is_binary());
    }
}
