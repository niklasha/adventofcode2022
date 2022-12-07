use crate::day::*;
use std::collections::HashMap;
use std::fmt::Debug;

pub struct Day07 {}

type Output = usize;

impl Day for Day07 {
    fn tag(&self) -> &str {
        "07"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

#[derive(Debug)]
enum Type {
    File,
    Directory(HashMap<String, usize>),
}

#[derive(Debug)]
struct Inode {
    size: usize,
    ty: Type,
}

impl Inode {
    fn new(ty: Type, size: usize) -> Self {
        Self { ty, size }
    }

    fn size(&self, fs: &FileSystem) -> Result<usize, AocError> {
        Ok(match &self.ty {
            Type::File => self.size,
            Type::Directory(map) => {
                self.size
                    + map
                        .iter()
                        .filter(|(name, _)| *name != "/" && *name != "." && *name != "..")
                        .map(|(_, ino)| fs.get(*ino).ok_or(AocError)?.size(fs))
                        .sum::<Result<usize, _>>()?
            }
        })
    }

    fn dfs_dir_fold(
        &self,
        fs: &FileSystem,
        init: usize, /*S*/ /* , f: F */
    ) -> Result<usize /*S*/, AocError>
// where
    //     F: Fn(&Inode, S) -> Result<S, AocError>,
    //     S: Debug,
    {
        let f = |inode: &Inode, total: usize| -> Result<usize, AocError> {
            inode
                .size(fs)
                .map(|size| if size <= 100000 { size + total } else { total })
        };
        match &self.ty {
            Type::File => Ok(init),
            Type::Directory(map) => {
                let state = f(self, init);
                map.iter()
                    .filter(|(name, _)| *name != "/" && *name != "." && *name != "..")
                    .fold(state, |state, (_name, ino)| {
                        let inode = fs.get(*ino).ok_or(AocError)?;
                        inode.dfs_dir_fold(fs, state? /*, f*/)
                    })
            }
        }
    }

    fn dfs2_dir_fold(
        &self,
        fs: &FileSystem,
        init: Option<usize>, /*S*/ /* , f: F */
    ) -> Result<Option<usize> /*S*/, AocError>
// where
    //     F: Fn(&Inode, S) -> Result<S, AocError>,
    //     S: Debug,
    {
        let f = |inode: &Inode, best: Option<usize>| -> Result<Option<usize>, AocError> {
            inode.size(fs).map(|size| {
                if size
                    >= 30000000
                        - (70000000
                            - fs.get(0)
                                .ok_or(AocError)
                                .and_then(|inode| inode.size(fs))
                                .ok()?)
                // XXX loses error info
                {
                    Some(if let Some(best) = best {
                        if size < best {
                            size
                        } else {
                            best
                        }
                    } else {
                        size
                    })
                } else {
                    best
                }
            })
        };
        match &self.ty {
            Type::File => Ok(init),
            Type::Directory(map) => {
                let state = f(self, init);
                map.iter()
                    .filter(|(name, _)| *name != "/" && *name != "." && *name != "..")
                    .fold(state, |state, (_name, ino)| {
                        let inode = fs.get(*ino).ok_or(AocError)?;
                        inode.dfs2_dir_fold(fs, state? /*, f*/)
                    })
            }
        }
    }
}

#[derive(Debug)]
struct FileSystem {
    inner: Vec<Inode>,
}

impl FileSystem {
    fn new() -> Self {
        let mut inner = vec![];
        let mut children = HashMap::new();
        children.insert("/".to_owned(), 0);
        children.insert(".".to_owned(), 0);
        children.insert("..".to_owned(), 0);
        let root = Inode::new(Type::Directory(children), 0);
        inner.push(root);
        FileSystem { inner }
    }

    fn get(&self, ino: usize) -> Option<&Inode> {
        self.inner.get(ino)
    }

    fn get_ino(&self, cwd: usize, name: &str) -> Result<usize, AocError> {
        let inode = self.inner.get(cwd).ok_or(AocError)?;
        if let Type::Directory(map) = &inode.ty {
            map.get(name).ok_or(AocError).map(|i| *i)
        } else {
            Err(AocError)?
        }
    }

    fn mkdir(&mut self, cwd: usize, name: &str) -> Result<(), AocError> {
        let next_ino = self.inner.len();
        let inode = self.inner.get_mut(cwd).ok_or(AocError)?;
        let dir = if let Type::Directory(map) = &mut inode.ty {
            let mut children = HashMap::new();
            children.insert(".".to_owned(), next_ino);
            children.insert("..".to_owned(), cwd);
            let dir = Inode::new(Type::Directory(children), 0);
            map.insert(name.to_owned(), next_ino);
            Ok(dir)
        } else {
            Err(AocError)
        }?;
        self.inner.push(dir);
        Ok(())
    }

    fn mkfile(&mut self, cwd: usize, name: &str, size: usize) -> Result<(), AocError> {
        let next_ino = self.inner.len();
        let inode = self.inner.get_mut(cwd).ok_or(AocError)?;
        let file = if let Type::Directory(map) = &mut inode.ty {
            let file = Inode::new(Type::File, size);
            map.insert(name.to_owned(), next_ino);
            Ok(file)
        } else {
            Err(AocError)
        }?;
        self.inner.push(file);
        Ok(())
    }

    fn dfs<F, S>(&self, init: usize /*S*/, _f: F) -> Result<usize /*S*/, AocError>
    where
        F: Fn(&Inode, S) -> Result<S, AocError>,
        S: Debug,
    {
        self.inner
            .get(0)
            .ok_or(AocError)
            .and_then(|root| root.dfs_dir_fold(self, init /* , f*/))
    }

    fn dfs2<F, S>(
        &self,
        init: Option<usize>, /*S*/
        _f: F,
    ) -> Result<Option<usize> /*S*/, AocError>
    where
        F: Fn(&Inode, S) -> Result<S, AocError>,
        S: Debug,
    {
        self.inner
            .get(0)
            .ok_or(AocError)
            .and_then(|root| root.dfs2_dir_fold(self, init /* , f*/))
    }
}

impl Day07 {
    fn parse(input: &mut dyn io::Read) -> BoxResult<FileSystem> {
        let lines = io::BufReader::new(input)
            .lines()
            .map(|l| l.map_err(|e| e.into()));
        let fs = lines.fold(
            Ok((FileSystem::new(), None)),
            |state: BoxResult<(FileSystem, Option<usize>)>, l: BoxResult<String>| {
                l.and_then(|l| {
                    if let Ok((mut fs, mut cwd)) = state {
                        let mut tokens = l.split_whitespace();
                        match tokens.next() {
                            Some("$") => match tokens.next() {
                                Some("cd") => {
                                    let name = tokens.next().ok_or(AocError)?;
                                    cwd = Some(if name == "/" {
                                        0
                                    } else {
                                        fs.get_ino(cwd.ok_or(AocError)?, name)?
                                    });
                                }
                                Some("ls") => (),
                                _ => Err(AocError)?,
                            },
                            Some("dir") => {
                                let name = tokens.next().ok_or(AocError)?;
                                fs.mkdir(cwd.ok_or(AocError)?, name)?;
                            }
                            Some(size) => {
                                let name = tokens.next().ok_or(AocError)?;
                                fs.mkfile(cwd.ok_or(AocError)?, name, size.parse()?)?;
                            }
                            _ => Err(AocError)?,
                        }
                        Ok((fs, cwd))
                    } else {
                        state
                    }
                })
            },
        );
        fs.map(|(fs, _)| fs)
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let fs = Self::parse(input)?;
        // println!("{:#?}", fs);
        let sizes = fs.dfs(0, |inode: &Inode, total: usize| {
            inode
                .size(&fs)
                .map(|size| if size <= 100000 { size + total } else { total })
        });
        sizes.map_err(|e| e.into())
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let fs = Self::parse(input)?;
        // find dir with at least at least 8381165 in size
        let sizes = fs.dfs2(
            None,
            |inode: &Inode, best: Option<usize>| -> Result<Option<usize>, AocError> {
                inode.size(&fs).map(|size| {
                    if size
                        < 30000000
                            - (70000000
                                - fs.get(0)
                                    .ok_or(AocError)
                                    .and_then(|inode| inode.size(&fs))
                                    .ok()?)
                    // XXX loses error info
                    {
                        Some(if let Some(best) = best {
                            if size < best {
                                size
                            } else {
                                best
                            }
                        } else {
                            size
                        })
                    } else {
                        best
                    }
                })
            },
        );
        Ok(sizes.unwrap().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day07 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
            95437,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day07 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
            24933642,
        );
    }
}
