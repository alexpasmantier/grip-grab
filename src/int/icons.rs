use std::{collections::HashMap, path::Path};

use lazy_static::lazy_static;

pub struct File<'a> {
    name: &'a str,
    ext: Option<String>,
}

impl File<'_> {
    pub fn new<'a>(path: &'a Path) -> File<'a> {
        let name = path.file_name().unwrap().to_str().unwrap();
        let ext = Self::ext(path);

        File { name, ext }
    }

    fn points_to_directory(&self) -> bool {
        self.name.ends_with('/')
    }

    fn ext(path: &Path) -> Option<String> {
        let name = path.file_name().map(|f| f.to_string_lossy().to_string())?;

        name.rfind('.').map(|p| name[p + 1..].to_ascii_lowercase())
    }
}

#[derive(Clone, Debug)]
pub struct FileIcon {
    pub icon: char,
    pub color: &'static str,
}

lazy_static! {
    static ref ICONS_MAP: HashMap<&'static str, FileIcon> = {
        let mut m = HashMap::new();
        m.insert(
            ".babelrc",
            FileIcon {
                icon: '',
                color: "#cbcb41",
            },
        );
        m.insert(
            ".bash_profile",
            FileIcon {
                icon: '',
                color: "#89e051",
            },
        );
        m.insert(
            ".bashrc",
            FileIcon {
                icon: '',
                color: "#89e051",
            },
        );
        m.insert(
            ".dockerignore",
            FileIcon {
                icon: '󰡨',
                color: "#458ee6",
            },
        );
        m.insert(
            ".ds_store",
            FileIcon {
                icon: '',
                color: "#41535b",
            },
        );
        m.insert(
            ".editorconfig",
            FileIcon {
                icon: '',
                color: "#fff2f2",
            },
        );
        m.insert(
            ".env",
            FileIcon {
                icon: '',
                color: "#faf743",
            },
        );
        m.insert(
            ".eslintrc",
            FileIcon {
                icon: '',
                color: "#4b32c3",
            },
        );
        m.insert(
            ".eslintignore",
            FileIcon {
                icon: '',
                color: "#4b32c3",
            },
        );
        m.insert(
            ".git-blame-ignore-revs",
            FileIcon {
                icon: '',
                color: "#f54d27",
            },
        );
        m.insert(
            ".gitattributes",
            FileIcon {
                icon: '',
                color: "#f54d27",
            },
        );
        m.insert(
            ".gitconfig",
            FileIcon {
                icon: '',
                color: "#f54d27",
            },
        );
        m.insert(
            ".gitignore",
            FileIcon {
                icon: '',
                color: "#f54d27",
            },
        );
        m.insert(
            ".gitlab-ci.yml",
            FileIcon {
                icon: '',
                color: "#e24329",
            },
        );
        m.insert(
            ".gitmodules",
            FileIcon {
                icon: '',
                color: "#f54d27",
            },
        );
        m.insert(
            ".gtkrc-2.0",
            FileIcon {
                icon: '',
                color: "#ffffff",
            },
        );
        m.insert(
            ".gvimrc",
            FileIcon {
                icon: '',
                color: "#019833",
            },
        );
        m.insert(
            ".justfile",
            FileIcon {
                icon: '',
                color: "#6d8086",
            },
        );
        m.insert(
            ".luaurc",
            FileIcon {
                icon: '',
                color: "#00a2ff",
            },
        );
        m.insert(
            ".mailmap",
            FileIcon {
                icon: '󰊢',
                color: "#f54d27",
            },
        );
        m.insert(
            ".npmignore",
            FileIcon {
                icon: '',
                color: "#E8274B",
            },
        );
        m.insert(
            ".npmrc",
            FileIcon {
                icon: '',
                color: "#E8274B",
            },
        );
        m.insert(
            ".nuxtrc",
            FileIcon {
                icon: '󱄆',
                color: "#00c58e",
            },
        );
        m.insert(
            ".nvmrc",
            FileIcon {
                icon: '',
                color: "#5FA04E",
            },
        );
        m.insert(
            ".prettierrc",
            FileIcon {
                icon: '',
                color: "#4285F4",
            },
        );
        m.insert(
            ".prettierrc.json",
            FileIcon {
                icon: '',
                color: "#4285F4",
            },
        );
        m.insert(
            ".prettierrc.json5",
            FileIcon {
                icon: '',
                color: "#4285F4",
            },
        );
        m.insert(
            ".prettierrc.toml",
            FileIcon {
                icon: '',
                color: "#4285F4",
            },
        );
        m.insert(
            ".prettierrc.yaml",
            FileIcon {
                icon: '',
                color: "#4285F4",
            },
        );
        m.insert(
            ".prettierrc.yml",
            FileIcon {
                icon: '',
                color: "#4285F4",
            },
        );
        m.insert(
            ".prettierignore",
            FileIcon {
                icon: '',
                color: "#4285F4",
            },
        );
        m.insert(
            ".settings.json",
            FileIcon {
                icon: '',
                color: "#854CC7",
            },
        );
        m.insert(
            ".SRCINFO",
            FileIcon {
                icon: '󰣇',
                color: "#0f94d2",
            },
        );
        m.insert(
            ".vimrc",
            FileIcon {
                icon: '',
                color: "#019833",
            },
        );
        m.insert(
            ".Xauthority",
            FileIcon {
                icon: '',
                color: "#e54d18",
            },
        );
        m.insert(
            ".xinitrc",
            FileIcon {
                icon: '',
                color: "#e54d18",
            },
        );
        m.insert(
            ".Xresources",
            FileIcon {
                icon: '',
                color: "#e54d18",
            },
        );
        m.insert(
            ".xsession",
            FileIcon {
                icon: '',
                color: "#e54d18",
            },
        );
        m.insert(
            ".zprofile",
            FileIcon {
                icon: '',
                color: "#89e051",
            },
        );
        m.insert(
            ".zshenv",
            FileIcon {
                icon: '',
                color: "#89e051",
            },
        );
        m.insert(
            ".zshrc",
            FileIcon {
                icon: '',
                color: "#89e051",
            },
        );
        m.insert(
            "_gvimrc",
            FileIcon {
                icon: '',
                color: "#019833",
            },
        );
        m.insert(
            "_vimrc",
            FileIcon {
                icon: '',
                color: "#019833",
            },
        );
        m.insert(
            "R",
            FileIcon {
                icon: '󰟔',
                color: "#2266ba",
            },
        );
        m.insert(
            "avif",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "brewfile",
            FileIcon {
                icon: '',
                color: "#701516",
            },
        );
        m.insert(
            "bspwmrc",
            FileIcon {
                icon: '',
                color: "#2f2f2f",
            },
        );
        m.insert(
            "build",
            FileIcon {
                icon: '',
                color: "#89e051",
            },
        );
        m.insert(
            "build.gradle",
            FileIcon {
                icon: '',
                color: "#005f87",
            },
        );
        m.insert(
            "build.zig.zon",
            FileIcon {
                icon: '',
                color: "#f69a1b",
            },
        );
        m.insert(
            "checkhealth",
            FileIcon {
                icon: '󰓙',
                color: "#75B4FB",
            },
        );
        m.insert(
            "cmakelists.txt",
            FileIcon {
                icon: '',
                color: "#6d8086",
            },
        );
        m.insert(
            "code_of_conduct",
            FileIcon {
                icon: '',
                color: "#E41662",
            },
        );
        m.insert(
            "code_of_conduct.md",
            FileIcon {
                icon: '',
                color: "#E41662",
            },
        );
        m.insert(
            "commit_editmsg",
            FileIcon {
                icon: '',
                color: "#f54d27",
            },
        );
        m.insert(
            "commitlint.config.js",
            FileIcon {
                icon: '󰜘',
                color: "#2b9689",
            },
        );
        m.insert(
            "commitlint.config.ts",
            FileIcon {
                icon: '󰜘',
                color: "#2b9689",
            },
        );
        m.insert(
            "compose.yaml",
            FileIcon {
                icon: '󰡨',
                color: "#458ee6",
            },
        );
        m.insert(
            "compose.yml",
            FileIcon {
                icon: '󰡨',
                color: "#458ee6",
            },
        );
        m.insert(
            "config",
            FileIcon {
                icon: '',
                color: "#6d8086",
            },
        );
        m.insert(
            "containerfile",
            FileIcon {
                icon: '󰡨',
                color: "#458ee6",
            },
        );
        m.insert(
            "copying",
            FileIcon {
                icon: '',
                color: "#cbcb41",
            },
        );
        m.insert(
            "copying.lesser",
            FileIcon {
                icon: '',
                color: "#cbcb41",
            },
        );
        m.insert(
            "docker-compose.yaml",
            FileIcon {
                icon: '󰡨',
                color: "#458ee6",
            },
        );
        m.insert(
            "docker-compose.yml",
            FileIcon {
                icon: '󰡨',
                color: "#458ee6",
            },
        );
        m.insert(
            "dockerfile",
            FileIcon {
                icon: '󰡨',
                color: "#458ee6",
            },
        );
        m.insert(
            "eslint.config.cjs",
            FileIcon {
                icon: '',
                color: "#4b32c3",
            },
        );
        m.insert(
            "eslint.config.js",
            FileIcon {
                icon: '',
                color: "#4b32c3",
            },
        );
        m.insert(
            "eslint.config.mjs",
            FileIcon {
                icon: '',
                color: "#4b32c3",
            },
        );
        m.insert(
            "eslint.config.ts",
            FileIcon {
                icon: '',
                color: "#4b32c3",
            },
        );
        m.insert(
            "ext_typoscript_setup.txt",
            FileIcon {
                icon: '',
                color: "#FF8700",
            },
        );
        m.insert(
            "favicon.ico",
            FileIcon {
                icon: '',
                color: "#cbcb41",
            },
        );
        m.insert(
            "fp-info-cache",
            FileIcon {
                icon: '',
                color: "#ffffff",
            },
        );
        m.insert(
            "fp-lib-table",
            FileIcon {
                icon: '',
                color: "#ffffff",
            },
        );
        m.insert(
            "FreeCAD.conf",
            FileIcon {
                icon: '',
                color: "#CB333B",
            },
        );
        m.insert(
            "gemfile$",
            FileIcon {
                icon: '',
                color: "#701516",
            },
        );
        m.insert(
            "gnumakefile",
            FileIcon {
                icon: '',
                color: "#6d8086",
            },
        );
        m.insert(
            "go.mod",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "go.sum",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "go.work",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "gradlew",
            FileIcon {
                icon: '',
                color: "#005f87",
            },
        );
        m.insert(
            "gradle.properties",
            FileIcon {
                icon: '',
                color: "#005f87",
            },
        );
        m.insert(
            "gradle-wrapper.properties",
            FileIcon {
                icon: '',
                color: "#005f87",
            },
        );
        m.insert(
            "groovy",
            FileIcon {
                icon: '',
                color: "#4a687c",
            },
        );
        m.insert(
            "gruntfile.babel.js",
            FileIcon {
                icon: '',
                color: "#e37933",
            },
        );
        m.insert(
            "gruntfile.coffee",
            FileIcon {
                icon: '',
                color: "#e37933",
            },
        );
        m.insert(
            "gruntfile.js",
            FileIcon {
                icon: '',
                color: "#e37933",
            },
        );
        m.insert(
            "gruntfile.ts",
            FileIcon {
                icon: '',
                color: "#e37933",
            },
        );
        m.insert(
            "gtkrc",
            FileIcon {
                icon: '',
                color: "#ffffff",
            },
        );
        m.insert(
            "gulpfile.babel.js",
            FileIcon {
                icon: '',
                color: "#cc3e44",
            },
        );
        m.insert(
            "gulpfile.coffee",
            FileIcon {
                icon: '',
                color: "#cc3e44",
            },
        );
        m.insert(
            "gulpfile.js",
            FileIcon {
                icon: '',
                color: "#cc3e44",
            },
        );
        m.insert(
            "gulpfile.ts",
            FileIcon {
                icon: '',
                color: "#cc3e44",
            },
        );
        m.insert(
            "hypridle.conf",
            FileIcon {
                icon: '',
                color: "#00aaae",
            },
        );
        m.insert(
            "hyprland.conf",
            FileIcon {
                icon: '',
                color: "#00aaae",
            },
        );
        m.insert(
            "hyprlock.conf",
            FileIcon {
                icon: '',
                color: "#00aaae",
            },
        );
        m.insert(
            "i18n.config.js",
            FileIcon {
                icon: '󰗊',
                color: "#7986cb",
            },
        );
        m.insert(
            "i18n.config.ts",
            FileIcon {
                icon: '󰗊',
                color: "#7986cb",
            },
        );
        m.insert(
            "i3blocks.conf",
            FileIcon {
                icon: '',
                color: "#e8ebee",
            },
        );
        m.insert(
            "i3status.conf",
            FileIcon {
                icon: '',
                color: "#e8ebee",
            },
        );
        m.insert(
            "ionic.config.json",
            FileIcon {
                icon: '',
                color: "#4f8ff7",
            },
        );
        m.insert(
            "cantorrc",
            FileIcon {
                icon: '',
                color: "#1c99f3",
            },
        );
        m.insert(
            "justfile",
            FileIcon {
                icon: '',
                color: "#6d8086",
            },
        );
        m.insert(
            "kalgebrarc",
            FileIcon {
                icon: '',
                color: "#1c99f3",
            },
        );
        m.insert(
            "kdeglobals",
            FileIcon {
                icon: '',
                color: "#1c99f3",
            },
        );
        m.insert(
            "kdenlive-layoutsrc",
            FileIcon {
                icon: '',
                color: "#83b8f2",
            },
        );
        m.insert(
            "kdenliverc",
            FileIcon {
                icon: '',
                color: "#83b8f2",
            },
        );
        m.insert(
            "kritadisplayrc",
            FileIcon {
                icon: '',
                color: "#f245fb",
            },
        );
        m.insert(
            "kritarc",
            FileIcon {
                icon: '',
                color: "#f245fb",
            },
        );
        m.insert(
            "license",
            FileIcon {
                icon: '',
                color: "#d0bf41",
            },
        );
        m.insert(
            "license.md",
            FileIcon {
                icon: '',
                color: "#d0bf41",
            },
        );
        m.insert(
            "lxde-rc.xml",
            FileIcon {
                icon: '',
                color: "#909090",
            },
        );
        m.insert(
            "lxqt.conf",
            FileIcon {
                icon: '',
                color: "#0192d3",
            },
        );
        m.insert(
            "makefile",
            FileIcon {
                icon: '',
                color: "#6d8086",
            },
        );
        m.insert(
            "mix.lock",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "mpv.conf",
            FileIcon {
                icon: '',
                color: "#3b1342",
            },
        );
        m.insert(
            "node_modules",
            FileIcon {
                icon: '',
                color: "#E8274B",
            },
        );
        m.insert(
            "nuxt.config.cjs",
            FileIcon {
                icon: '󱄆',
                color: "#00c58e",
            },
        );
        m.insert(
            "nuxt.config.js",
            FileIcon {
                icon: '󱄆',
                color: "#00c58e",
            },
        );
        m.insert(
            "nuxt.config.mjs",
            FileIcon {
                icon: '󱄆',
                color: "#00c58e",
            },
        );
        m.insert(
            "nuxt.config.ts",
            FileIcon {
                icon: '󱄆',
                color: "#00c58e",
            },
        );
        m.insert(
            "package.json",
            FileIcon {
                icon: '',
                color: "#e8274b",
            },
        );
        m.insert(
            "package-lock.json",
            FileIcon {
                icon: '',
                color: "#7a0d21",
            },
        );
        m.insert(
            "PKGBUILD",
            FileIcon {
                icon: '',
                color: "#0f94d2",
            },
        );
        m.insert(
            "platformio.ini",
            FileIcon {
                icon: '',
                color: "#f6822b",
            },
        );
        m.insert(
            "pom.xml",
            FileIcon {
                icon: '',
                color: "#7a0d21",
            },
        );
        m.insert(
            "prettier.config.js",
            FileIcon {
                icon: '',
                color: "#4285F4",
            },
        );
        m.insert(
            "prettier.config.cjs",
            FileIcon {
                icon: '',
                color: "#4285F4",
            },
        );
        m.insert(
            "prettier.config.mjs",
            FileIcon {
                icon: '',
                color: "#4285F4",
            },
        );
        m.insert(
            "prettier.config.ts",
            FileIcon {
                icon: '',
                color: "#4285F4",
            },
        );
        m.insert(
            "procfile",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "PrusaSlicer.ini",
            FileIcon {
                icon: '',
                color: "#ec6b23",
            },
        );
        m.insert(
            "PrusaSlicerGcodeViewer.ini",
            FileIcon {
                icon: '',
                color: "#ec6b23",
            },
        );
        m.insert(
            "py.typed",
            FileIcon {
                icon: '',
                color: "#ffbc03",
            },
        );
        m.insert(
            "QtProject.conf",
            FileIcon {
                icon: '',
                color: "#40cd52",
            },
        );
        m.insert(
            "r",
            FileIcon {
                icon: '󰟔',
                color: "#2266ba",
            },
        );
        m.insert(
            "rakefile",
            FileIcon {
                icon: '',
                color: "#701516",
            },
        );
        m.insert(
            "rmd",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "robots.txt",
            FileIcon {
                icon: '󰚩',
                color: "#5d7096",
            },
        );
        m.insert(
            "security",
            FileIcon {
                icon: '󰒃',
                color: "#BEC4C9",
            },
        );
        m.insert(
            "security.md",
            FileIcon {
                icon: '󰒃',
                color: "#BEC4C9",
            },
        );
        m.insert(
            "settings.gradle",
            FileIcon {
                icon: '',
                color: "#005f87",
            },
        );
        m.insert(
            "svelte.config.js",
            FileIcon {
                icon: '',
                color: "#ff3e00",
            },
        );
        m.insert(
            "sxhkdrc",
            FileIcon {
                icon: '',
                color: "#2f2f2f",
            },
        );
        m.insert(
            "sym-lib-table",
            FileIcon {
                icon: '',
                color: "#ffffff",
            },
        );
        m.insert(
            "tailwind.config.js",
            FileIcon {
                icon: '󱏿',
                color: "#20c2e3",
            },
        );
        m.insert(
            "tailwind.config.mjs",
            FileIcon {
                icon: '󱏿',
                color: "#20c2e3",
            },
        );
        m.insert(
            "tailwind.config.ts",
            FileIcon {
                icon: '󱏿',
                color: "#20c2e3",
            },
        );
        m.insert(
            "tmux.conf",
            FileIcon {
                icon: '',
                color: "#14ba19",
            },
        );
        m.insert(
            "tmux.conf.local",
            FileIcon {
                icon: '',
                color: "#14ba19",
            },
        );
        m.insert(
            "tsconfig.json",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "unlicense",
            FileIcon {
                icon: '',
                color: "#d0bf41",
            },
        );
        m.insert(
            "vagrantfile$",
            FileIcon {
                icon: '',
                color: "#1563FF",
            },
        );
        m.insert(
            "vlcrc",
            FileIcon {
                icon: '󰕼',
                color: "#ee7a00",
            },
        );
        m.insert(
            "vercel.json",
            FileIcon {
                icon: '▲',
                color: "#ffffff",
            },
        );
        m.insert(
            "webpack",
            FileIcon {
                icon: '󰜫',
                color: "#519aba",
            },
        );
        m.insert(
            "weston.ini",
            FileIcon {
                icon: '',
                color: "#ffbb01",
            },
        );
        m.insert(
            "workspace",
            FileIcon {
                icon: '',
                color: "#89e051",
            },
        );
        m.insert(
            "xmobarrc",
            FileIcon {
                icon: '',
                color: "#fd4d5d",
            },
        );
        m.insert(
            "xmobarrc.hs",
            FileIcon {
                icon: '',
                color: "#fd4d5d",
            },
        );
        m.insert(
            "xmonad.hs",
            FileIcon {
                icon: '',
                color: "#fd4d5d",
            },
        );
        m.insert(
            "xorg.conf",
            FileIcon {
                icon: '',
                color: "#e54d18",
            },
        );
        m.insert(
            "xsettingsd.conf",
            FileIcon {
                icon: '',
                color: "#e54d18",
            },
        );
        m.insert(
            "3gp",
            FileIcon {
                icon: '',
                color: "#FD971F",
            },
        );
        m.insert(
            "3mf",
            FileIcon {
                icon: '󰆧',
                color: "#888888",
            },
        );
        m.insert(
            "7z",
            FileIcon {
                icon: '',
                color: "#eca517",
            },
        );
        m.insert(
            "a",
            FileIcon {
                icon: '',
                color: "#dcddd6",
            },
        );
        m.insert(
            "aac",
            FileIcon {
                icon: '',
                color: "#00afff",
            },
        );
        m.insert(
            "aif",
            FileIcon {
                icon: '',
                color: "#00afff",
            },
        );
        m.insert(
            "aiff",
            FileIcon {
                icon: '',
                color: "#00afff",
            },
        );
        m.insert(
            "ape",
            FileIcon {
                icon: '',
                color: "#00afff",
            },
        );
        m.insert(
            "apl",
            FileIcon {
                icon: '⍝',
                color: "#ffa500",
            },
        );
        m.insert(
            "ai",
            FileIcon {
                icon: '',
                color: "#cbcb41",
            },
        );
        m.insert(
            "android",
            FileIcon {
                icon: '',
                color: "#34a853",
            },
        );
        m.insert(
            "apk",
            FileIcon {
                icon: '',
                color: "#34a853",
            },
        );
        m.insert(
            "app",
            FileIcon {
                icon: '',
                color: "#9F0500",
            },
        );
        m.insert(
            "applescript",
            FileIcon {
                icon: '',
                color: "#6d8085",
            },
        );
        m.insert(
            "asc",
            FileIcon {
                icon: '󰦝',
                color: "#576d7f",
            },
        );
        m.insert(
            "ass",
            FileIcon {
                icon: '󰨖',
                color: "#ffb713",
            },
        );
        m.insert(
            "astro",
            FileIcon {
                icon: '',
                color: "#e23f67",
            },
        );
        m.insert(
            "awk",
            FileIcon {
                icon: '',
                color: "#4d5a5e",
            },
        );
        m.insert(
            "azcli",
            FileIcon {
                icon: '',
                color: "#0078d4",
            },
        );
        m.insert(
            "bak",
            FileIcon {
                icon: '󰁯',
                color: "#6d8086",
            },
        );
        m.insert(
            "bash",
            FileIcon {
                icon: '',
                color: "#89e051",
            },
        );
        m.insert(
            "bat",
            FileIcon {
                icon: '',
                color: "#C1F12E",
            },
        );
        m.insert(
            "bazel",
            FileIcon {
                icon: '',
                color: "#89e051",
            },
        );
        m.insert(
            "bib",
            FileIcon {
                icon: '󱉟',
                color: "#cbcb41",
            },
        );
        m.insert(
            "bicep",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "bicepparam",
            FileIcon {
                icon: '',
                color: "#9f74b3",
            },
        );
        m.insert(
            "bin",
            FileIcon {
                icon: '',
                color: "#9F0500",
            },
        );
        m.insert(
            "blade.php",
            FileIcon {
                icon: '',
                color: "#f05340",
            },
        );
        m.insert(
            "blend",
            FileIcon {
                icon: '󰂫',
                color: "#ea7600",
            },
        );
        m.insert(
            "bmp",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "bqn",
            FileIcon {
                icon: '⎉',
                color: "#2b7067",
            },
        );
        m.insert(
            "blp",
            FileIcon {
                icon: '󰺾',
                color: "#5796E2",
            },
        );
        m.insert(
            "brep",
            FileIcon {
                icon: '󰻫',
                color: "#839463",
            },
        );
        m.insert(
            "bz",
            FileIcon {
                icon: '',
                color: "#eca517",
            },
        );
        m.insert(
            "bz2",
            FileIcon {
                icon: '',
                color: "#eca517",
            },
        );
        m.insert(
            "bz3",
            FileIcon {
                icon: '',
                color: "#eca517",
            },
        );
        m.insert(
            "bzl",
            FileIcon {
                icon: '',
                color: "#89e051",
            },
        );
        m.insert(
            "c",
            FileIcon {
                icon: '',
                color: "#599eff",
            },
        );
        m.insert(
            "c++",
            FileIcon {
                icon: '',
                color: "#f34b7d",
            },
        );
        m.insert(
            "cache",
            FileIcon {
                icon: '',
                color: "#ffffff",
            },
        );
        m.insert(
            "cast",
            FileIcon {
                icon: '',
                color: "#FD971F",
            },
        );
        m.insert(
            "cbl",
            FileIcon {
                icon: '⚙',
                color: "#005ca5",
            },
        );
        m.insert(
            "cc",
            FileIcon {
                icon: '',
                color: "#f34b7d",
            },
        );
        m.insert(
            "ccm",
            FileIcon {
                icon: '',
                color: "#f34b7d",
            },
        );
        m.insert(
            "cfg",
            FileIcon {
                icon: '',
                color: "#6d8086",
            },
        );
        m.insert(
            "cjs",
            FileIcon {
                icon: '',
                color: "#cbcb41",
            },
        );
        m.insert(
            "clj",
            FileIcon {
                icon: '',
                color: "#8dc149",
            },
        );
        m.insert(
            "cljc",
            FileIcon {
                icon: '',
                color: "#8dc149",
            },
        );
        m.insert(
            "cljs",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "cljd",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "cmake",
            FileIcon {
                icon: '',
                color: "#6d8086",
            },
        );
        m.insert(
            "cob",
            FileIcon {
                icon: '⚙',
                color: "#005ca5",
            },
        );
        m.insert(
            "cobol",
            FileIcon {
                icon: '⚙',
                color: "#005ca5",
            },
        );
        m.insert(
            "coffee",
            FileIcon {
                icon: '',
                color: "#cbcb41",
            },
        );
        m.insert(
            "conf",
            FileIcon {
                icon: '',
                color: "#6d8086",
            },
        );
        m.insert(
            "config.ru",
            FileIcon {
                icon: '',
                color: "#701516",
            },
        );
        m.insert(
            "cow",
            FileIcon {
                icon: '󰆚',
                color: "#965824",
            },
        );
        m.insert(
            "cp",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "cpp",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "cppm",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "cpy",
            FileIcon {
                icon: '⚙',
                color: "#005ca5",
            },
        );
        m.insert(
            "cr",
            FileIcon {
                icon: '',
                color: "#c8c8c8",
            },
        );
        m.insert(
            "crdownload",
            FileIcon {
                icon: '',
                color: "#44cda8",
            },
        );
        m.insert(
            "cs",
            FileIcon {
                icon: '󰌛',
                color: "#596706",
            },
        );
        m.insert(
            "csh",
            FileIcon {
                icon: '',
                color: "#4d5a5e",
            },
        );
        m.insert(
            "cshtml",
            FileIcon {
                icon: '󱦗',
                color: "#512bd4",
            },
        );
        m.insert(
            "cson",
            FileIcon {
                icon: '',
                color: "#cbcb41",
            },
        );
        m.insert(
            "csproj",
            FileIcon {
                icon: '󰪮',
                color: "#512bd4",
            },
        );
        m.insert(
            "css",
            FileIcon {
                icon: '',
                color: "#42a5f5",
            },
        );
        m.insert(
            "csv",
            FileIcon {
                icon: '',
                color: "#89e051",
            },
        );
        m.insert(
            "cts",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "cu",
            FileIcon {
                icon: '',
                color: "#89e051",
            },
        );
        m.insert(
            "cue",
            FileIcon {
                icon: '󰲹',
                color: "#ed95ae",
            },
        );
        m.insert(
            "cuh",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "cxx",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "cxxm",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "d",
            FileIcon {
                icon: '',
                color: "#427819",
            },
        );
        m.insert(
            "d.ts",
            FileIcon {
                icon: '',
                color: "#d59855",
            },
        );
        m.insert(
            "dart",
            FileIcon {
                icon: '',
                color: "#03589C",
            },
        );
        m.insert(
            "db",
            FileIcon {
                icon: '',
                color: "#dad8d8",
            },
        );
        m.insert(
            "dconf",
            FileIcon {
                icon: '',
                color: "#ffffff",
            },
        );
        m.insert(
            "desktop",
            FileIcon {
                icon: '',
                color: "#563d7c",
            },
        );
        m.insert(
            "diff",
            FileIcon {
                icon: '',
                color: "#41535b",
            },
        );
        m.insert(
            "dll",
            FileIcon {
                icon: '',
                color: "#4d2c0b",
            },
        );
        m.insert(
            "doc",
            FileIcon {
                icon: '󰈬',
                color: "#185abd",
            },
        );
        m.insert(
            "Dockerfile",
            FileIcon {
                icon: '󰡨',
                color: "#458ee6",
            },
        );
        m.insert(
            "docx",
            FileIcon {
                icon: '󰈬',
                color: "#185abd",
            },
        );
        m.insert(
            "dot",
            FileIcon {
                icon: '󱁉',
                color: "#30638e",
            },
        );
        m.insert(
            "download",
            FileIcon {
                icon: '',
                color: "#44cda8",
            },
        );
        m.insert(
            "drl",
            FileIcon {
                icon: '',
                color: "#ffafaf",
            },
        );
        m.insert(
            "dropbox",
            FileIcon {
                icon: '',
                color: "#0061FE",
            },
        );
        m.insert(
            "dump",
            FileIcon {
                icon: '',
                color: "#dad8d8",
            },
        );
        m.insert(
            "dwg",
            FileIcon {
                icon: '󰻫',
                color: "#839463",
            },
        );
        m.insert(
            "dxf",
            FileIcon {
                icon: '󰻫',
                color: "#839463",
            },
        );
        m.insert(
            "ebook",
            FileIcon {
                icon: '',
                color: "#eab16d",
            },
        );
        m.insert(
            "edn",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "eex",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "ejs",
            FileIcon {
                icon: '',
                color: "#cbcb41",
            },
        );
        m.insert(
            "elf",
            FileIcon {
                icon: '',
                color: "#9F0500",
            },
        );
        m.insert(
            "el",
            FileIcon {
                icon: '',
                color: "#8172be",
            },
        );
        m.insert(
            "elc",
            FileIcon {
                icon: '',
                color: "#8172be",
            },
        );
        m.insert(
            "elm",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "eln",
            FileIcon {
                icon: '',
                color: "#8172be",
            },
        );
        m.insert(
            "env",
            FileIcon {
                icon: '',
                color: "#faf743",
            },
        );
        m.insert(
            "eot",
            FileIcon {
                icon: '',
                color: "#ECECEC",
            },
        );
        m.insert(
            "epp",
            FileIcon {
                icon: '',
                color: "#FFA61A",
            },
        );
        m.insert(
            "epub",
            FileIcon {
                icon: '',
                color: "#eab16d",
            },
        );
        m.insert(
            "erb",
            FileIcon {
                icon: '',
                color: "#701516",
            },
        );
        m.insert(
            "erl",
            FileIcon {
                icon: '',
                color: "#B83998",
            },
        );
        m.insert(
            "ex",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "exe",
            FileIcon {
                icon: '',
                color: "#9F0500",
            },
        );
        m.insert(
            "exs",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "f#",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "f3d",
            FileIcon {
                icon: '󰻫',
                color: "#839463",
            },
        );
        m.insert(
            "f90",
            FileIcon {
                icon: '󱈚',
                color: "#734f96",
            },
        );
        m.insert(
            "fbx",
            FileIcon {
                icon: '󰆧',
                color: "#888888",
            },
        );
        m.insert(
            "fcbak",
            FileIcon {
                icon: '',
                color: "#cb333b",
            },
        );
        m.insert(
            "fcmacro",
            FileIcon {
                icon: '',
                color: "#cb333b",
            },
        );
        m.insert(
            "fcmat",
            FileIcon {
                icon: '',
                color: "#cb333b",
            },
        );
        m.insert(
            "fcparam",
            FileIcon {
                icon: '',
                color: "#cb333b",
            },
        );
        m.insert(
            "fcscript",
            FileIcon {
                icon: '',
                color: "#cb333b",
            },
        );
        m.insert(
            "fcstd",
            FileIcon {
                icon: '',
                color: "#cb333b",
            },
        );
        m.insert(
            "fcstd1",
            FileIcon {
                icon: '',
                color: "#cb333b",
            },
        );
        m.insert(
            "fctb",
            FileIcon {
                icon: '',
                color: "#cb333b",
            },
        );
        m.insert(
            "fctl",
            FileIcon {
                icon: '',
                color: "#cb333b",
            },
        );
        m.insert(
            "fdmdownload",
            FileIcon {
                icon: '',
                color: "#44cda8",
            },
        );
        m.insert(
            "flac",
            FileIcon {
                icon: '',
                color: "#0075aa",
            },
        );
        m.insert(
            "flc",
            FileIcon {
                icon: '',
                color: "#ECECEC",
            },
        );
        m.insert(
            "flf",
            FileIcon {
                icon: '',
                color: "#ECECEC",
            },
        );
        m.insert(
            "fnl",
            FileIcon {
                icon: '',
                color: "#fff3d7",
            },
        );
        m.insert(
            "fish",
            FileIcon {
                icon: '',
                color: "#4d5a5e",
            },
        );
        m.insert(
            "fs",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "fsi",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "fsscript",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "fsx",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "gcode",
            FileIcon {
                icon: '󰐫',
                color: "#1471ad",
            },
        );
        m.insert(
            "gd",
            FileIcon {
                icon: '',
                color: "#6d8086",
            },
        );
        m.insert(
            "gemspec",
            FileIcon {
                icon: '',
                color: "#701516",
            },
        );
        m.insert(
            "gif",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "git",
            FileIcon {
                icon: '',
                color: "#F14C28",
            },
        );
        m.insert(
            "glb",
            FileIcon {
                icon: '',
                color: "#FFB13B",
            },
        );
        m.insert(
            "gleam",
            FileIcon {
                icon: '',
                color: "#ffaff3",
            },
        );
        m.insert(
            "gnumakefile",
            FileIcon {
                icon: '',
                color: "#6d8086",
            },
        );
        m.insert(
            "go",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "godot",
            FileIcon {
                icon: '',
                color: "#6d8086",
            },
        );
        m.insert(
            "gql",
            FileIcon {
                icon: '',
                color: "#e535ab",
            },
        );
        m.insert(
            "gradle",
            FileIcon {
                icon: '',
                color: "#005f87",
            },
        );
        m.insert(
            "graphql",
            FileIcon {
                icon: '',
                color: "#e535ab",
            },
        );
        m.insert(
            "gresource",
            FileIcon {
                icon: '',
                color: "#ffffff",
            },
        );
        m.insert(
            "gv",
            FileIcon {
                icon: '󱁉',
                color: "#30638e",
            },
        );
        m.insert(
            "gz",
            FileIcon {
                icon: '',
                color: "#eca517",
            },
        );
        m.insert(
            "h",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "haml",
            FileIcon {
                icon: '',
                color: "#eaeae1",
            },
        );
        m.insert(
            "hx",
            FileIcon {
                icon: '',
                color: "#ea8220",
            },
        );
        m.insert(
            "hbs",
            FileIcon {
                icon: '',
                color: "#f0772b",
            },
        );
        m.insert(
            "hex",
            FileIcon {
                icon: '',
                color: "#2e63ff",
            },
        );
        m.insert(
            "heex",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "hh",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "hpp",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "hrl",
            FileIcon {
                icon: '',
                color: "#B83998",
            },
        );
        m.insert(
            "hs",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "htm",
            FileIcon {
                icon: '',
                color: "#e34c26",
            },
        );
        m.insert(
            "html",
            FileIcon {
                icon: '',
                color: "#e44d26",
            },
        );
        m.insert(
            "huff",
            FileIcon {
                icon: '󰡘',
                color: "#4242c7",
            },
        );
        m.insert(
            "hurl",
            FileIcon {
                icon: '',
                color: "#ff0288",
            },
        );
        m.insert(
            "hxx",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "ixx",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "ico",
            FileIcon {
                icon: '',
                color: "#cbcb41",
            },
        );
        m.insert(
            "ical",
            FileIcon {
                icon: '',
                color: "#2B2e83",
            },
        );
        m.insert(
            "icalendar",
            FileIcon {
                icon: '',
                color: "#2B2e83",
            },
        );
        m.insert(
            "ics",
            FileIcon {
                icon: '',
                color: "#2B2e83",
            },
        );
        m.insert(
            "ifb",
            FileIcon {
                icon: '',
                color: "#2B2e83",
            },
        );
        m.insert(
            "ifc",
            FileIcon {
                icon: '󰻫',
                color: "#839463",
            },
        );
        m.insert(
            "ige",
            FileIcon {
                icon: '󰻫',
                color: "#839463",
            },
        );
        m.insert(
            "iges",
            FileIcon {
                icon: '󰻫',
                color: "#839463",
            },
        );
        m.insert(
            "igs",
            FileIcon {
                icon: '󰻫',
                color: "#839463",
            },
        );
        m.insert(
            "image",
            FileIcon {
                icon: '',
                color: "#d0bec8",
            },
        );
        m.insert(
            "img",
            FileIcon {
                icon: '',
                color: "#d0bec8",
            },
        );
        m.insert(
            "import",
            FileIcon {
                icon: '',
                color: "#ECECEC",
            },
        );
        m.insert(
            "info",
            FileIcon {
                icon: '',
                color: "#ffffcd",
            },
        );
        m.insert(
            "ini",
            FileIcon {
                icon: '',
                color: "#6d8086",
            },
        );
        m.insert(
            "ino",
            FileIcon {
                icon: '',
                color: "#56b6c2",
            },
        );
        m.insert(
            "iso",
            FileIcon {
                icon: '',
                color: "#d0bec8",
            },
        );
        m.insert(
            "ipynb",
            FileIcon {
                icon: '',
                color: "#51a0cf",
            },
        );
        m.insert(
            "java",
            FileIcon {
                icon: '',
                color: "#cc3e44",
            },
        );
        m.insert(
            "jl",
            FileIcon {
                icon: '',
                color: "#a270ba",
            },
        );
        m.insert(
            "jwmrc",
            FileIcon {
                icon: '',
                color: "#0078cd",
            },
        );
        m.insert(
            "jpeg",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "jpg",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "js",
            FileIcon {
                icon: '',
                color: "#cbcb41",
            },
        );
        m.insert(
            "json",
            FileIcon {
                icon: '',
                color: "#cbcb41",
            },
        );
        m.insert(
            "json5",
            FileIcon {
                icon: '',
                color: "#cbcb41",
            },
        );
        m.insert(
            "jsonc",
            FileIcon {
                icon: '',
                color: "#cbcb41",
            },
        );
        m.insert(
            "jsx",
            FileIcon {
                icon: '',
                color: "#20c2e3",
            },
        );
        m.insert(
            "jxl",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "kbx",
            FileIcon {
                icon: '󰯄',
                color: "#737672",
            },
        );
        m.insert(
            "kdb",
            FileIcon {
                icon: '',
                color: "#529b34",
            },
        );
        m.insert(
            "kdbx",
            FileIcon {
                icon: '',
                color: "#529b34",
            },
        );
        m.insert(
            "kdenlive",
            FileIcon {
                icon: '',
                color: "#83b8f2",
            },
        );
        m.insert(
            "kdenlivetitle",
            FileIcon {
                icon: '',
                color: "#83b8f2",
            },
        );
        m.insert(
            "kicad_dru",
            FileIcon {
                icon: '',
                color: "#ffffff",
            },
        );
        m.insert(
            "kicad_mod",
            FileIcon {
                icon: '',
                color: "#ffffff",
            },
        );
        m.insert(
            "kicad_pcb",
            FileIcon {
                icon: '',
                color: "#ffffff",
            },
        );
        m.insert(
            "kicad_prl",
            FileIcon {
                icon: '',
                color: "#ffffff",
            },
        );
        m.insert(
            "kicad_pro",
            FileIcon {
                icon: '',
                color: "#ffffff",
            },
        );
        m.insert(
            "kicad_sch",
            FileIcon {
                icon: '',
                color: "#ffffff",
            },
        );
        m.insert(
            "kicad_sym",
            FileIcon {
                icon: '',
                color: "#ffffff",
            },
        );
        m.insert(
            "kicad_wks",
            FileIcon {
                icon: '',
                color: "#ffffff",
            },
        );
        m.insert(
            "ko",
            FileIcon {
                icon: '',
                color: "#dcddd6",
            },
        );
        m.insert(
            "kpp",
            FileIcon {
                icon: '',
                color: "#f245fb",
            },
        );
        m.insert(
            "kra",
            FileIcon {
                icon: '',
                color: "#f245fb",
            },
        );
        m.insert(
            "krz",
            FileIcon {
                icon: '',
                color: "#f245fb",
            },
        );
        m.insert(
            "ksh",
            FileIcon {
                icon: '',
                color: "#4d5a5e",
            },
        );
        m.insert(
            "kt",
            FileIcon {
                icon: '',
                color: "#7F52FF",
            },
        );
        m.insert(
            "kts",
            FileIcon {
                icon: '',
                color: "#7F52FF",
            },
        );
        m.insert(
            "lck",
            FileIcon {
                icon: '',
                color: "#bbbbbb",
            },
        );
        m.insert(
            "leex",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "less",
            FileIcon {
                icon: '',
                color: "#563d7c",
            },
        );
        m.insert(
            "lff",
            FileIcon {
                icon: '',
                color: "#ECECEC",
            },
        );
        m.insert(
            "lhs",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "lib",
            FileIcon {
                icon: '',
                color: "#4d2c0b",
            },
        );
        m.insert(
            "license",
            FileIcon {
                icon: '',
                color: "#cbcb41",
            },
        );
        m.insert(
            "liquid",
            FileIcon {
                icon: '',
                color: "#95BF47",
            },
        );
        m.insert(
            "lock",
            FileIcon {
                icon: '',
                color: "#bbbbbb",
            },
        );
        m.insert(
            "log",
            FileIcon {
                icon: '󰌱',
                color: "#dddddd",
            },
        );
        m.insert(
            "lrc",
            FileIcon {
                icon: '󰨖',
                color: "#ffb713",
            },
        );
        m.insert(
            "lua",
            FileIcon {
                icon: '',
                color: "#51a0cf",
            },
        );
        m.insert(
            "luac",
            FileIcon {
                icon: '',
                color: "#51a0cf",
            },
        );
        m.insert(
            "luau",
            FileIcon {
                icon: '',
                color: "#00a2ff",
            },
        );
        m.insert(
            "m3u",
            FileIcon {
                icon: '󰲹',
                color: "#ed95ae",
            },
        );
        m.insert(
            "m3u8",
            FileIcon {
                icon: '󰲹',
                color: "#ed95ae",
            },
        );
        m.insert(
            "m4a",
            FileIcon {
                icon: '',
                color: "#00afff",
            },
        );
        m.insert(
            "m4v",
            FileIcon {
                icon: '',
                color: "#FD971F",
            },
        );
        m.insert(
            "magnet",
            FileIcon {
                icon: '',
                color: "#a51b16",
            },
        );
        m.insert(
            "makefile",
            FileIcon {
                icon: '',
                color: "#6d8086",
            },
        );
        m.insert(
            "markdown",
            FileIcon {
                icon: '',
                color: "#dddddd",
            },
        );
        m.insert(
            "material",
            FileIcon {
                icon: '󰔉',
                color: "#B83998",
            },
        );
        m.insert(
            "md",
            FileIcon {
                icon: '',
                color: "#dddddd",
            },
        );
        m.insert(
            "md5",
            FileIcon {
                icon: '󰕥',
                color: "#8c86af",
            },
        );
        m.insert(
            "mdx",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "mint",
            FileIcon {
                icon: '󰌪',
                color: "#87c095",
            },
        );
        m.insert(
            "mjs",
            FileIcon {
                icon: '',
                color: "#f1e05a",
            },
        );
        m.insert(
            "mk",
            FileIcon {
                icon: '',
                color: "#6d8086",
            },
        );
        m.insert(
            "mkv",
            FileIcon {
                icon: '',
                color: "#FD971F",
            },
        );
        m.insert(
            "ml",
            FileIcon {
                icon: '',
                color: "#e37933",
            },
        );
        m.insert(
            "mli",
            FileIcon {
                icon: '',
                color: "#e37933",
            },
        );
        m.insert(
            "m",
            FileIcon {
                icon: '',
                color: "#599eff",
            },
        );
        m.insert(
            "mm",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "mo",
            FileIcon {
                icon: '∞',
                color: "#9772FB",
            },
        );
        m.insert(
            "mobi",
            FileIcon {
                icon: '',
                color: "#eab16d",
            },
        );
        m.insert(
            "mojo",
            FileIcon {
                icon: '',
                color: "#ff4c1f",
            },
        );
        m.insert(
            "🔥",
            FileIcon {
                icon: '',
                color: "#ff4c1f",
            },
        );
        m.insert(
            "mov",
            FileIcon {
                icon: '',
                color: "#FD971F",
            },
        );
        m.insert(
            "mp3",
            FileIcon {
                icon: '',
                color: "#00afff",
            },
        );
        m.insert(
            "mp4",
            FileIcon {
                icon: '',
                color: "#FD971F",
            },
        );
        m.insert(
            "mpp",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "msf",
            FileIcon {
                icon: '',
                color: "#137be1",
            },
        );
        m.insert(
            "mts",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "mustache",
            FileIcon {
                icon: '',
                color: "#e37933",
            },
        );
        m.insert(
            "nfo",
            FileIcon {
                icon: '',
                color: "#ffffcd",
            },
        );
        m.insert(
            "nim",
            FileIcon {
                icon: '',
                color: "#f3d400",
            },
        );
        m.insert(
            "nix",
            FileIcon {
                icon: '',
                color: "#7ebae4",
            },
        );
        m.insert(
            "nswag",
            FileIcon {
                icon: '',
                color: "#85ea2d",
            },
        );
        m.insert(
            "nu",
            FileIcon {
                icon: '>',
                color: "#3aa675",
            },
        );
        m.insert(
            "o",
            FileIcon {
                icon: '',
                color: "#9F0500",
            },
        );
        m.insert(
            "obj",
            FileIcon {
                icon: '󰆧',
                color: "#888888",
            },
        );
        m.insert(
            "ogg",
            FileIcon {
                icon: '',
                color: "#0075aa",
            },
        );
        m.insert(
            "opus",
            FileIcon {
                icon: '',
                color: "#0075aa",
            },
        );
        m.insert(
            "org",
            FileIcon {
                icon: '',
                color: "#77AA99",
            },
        );
        m.insert(
            "otf",
            FileIcon {
                icon: '',
                color: "#ECECEC",
            },
        );
        m.insert(
            "out",
            FileIcon {
                icon: '',
                color: "#9F0500",
            },
        );
        m.insert(
            "part",
            FileIcon {
                icon: '',
                color: "#44cda8",
            },
        );
        m.insert(
            "patch",
            FileIcon {
                icon: '',
                color: "#41535b",
            },
        );
        m.insert(
            "pck",
            FileIcon {
                icon: '',
                color: "#6d8086",
            },
        );
        m.insert(
            "pcm",
            FileIcon {
                icon: '',
                color: "#0075aa",
            },
        );
        m.insert(
            "pdf",
            FileIcon {
                icon: '',
                color: "#b30b00",
            },
        );
        m.insert(
            "php",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "pl",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "pls",
            FileIcon {
                icon: '󰲹',
                color: "#ed95ae",
            },
        );
        m.insert(
            "ply",
            FileIcon {
                icon: '󰆧',
                color: "#888888",
            },
        );
        m.insert(
            "pm",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "png",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "po",
            FileIcon {
                icon: '',
                color: "#2596be",
            },
        );
        m.insert(
            "pot",
            FileIcon {
                icon: '',
                color: "#2596be",
            },
        );
        m.insert(
            "pp",
            FileIcon {
                icon: '',
                color: "#FFA61A",
            },
        );
        m.insert(
            "ppt",
            FileIcon {
                icon: '󰈧',
                color: "#cb4a32",
            },
        );
        m.insert(
            "prisma",
            FileIcon {
                icon: '',
                color: "#5a67d8",
            },
        );
        m.insert(
            "pro",
            FileIcon {
                icon: '',
                color: "#e4b854",
            },
        );
        m.insert(
            "ps1",
            FileIcon {
                icon: '󰨊',
                color: "#4273ca",
            },
        );
        m.insert(
            "psd1",
            FileIcon {
                icon: '󰨊',
                color: "#6975c4",
            },
        );
        m.insert(
            "psm1",
            FileIcon {
                icon: '󰨊',
                color: "#6975c4",
            },
        );
        m.insert(
            "psb",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "psd",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "pub",
            FileIcon {
                icon: '󰷖',
                color: "#e3c58e",
            },
        );
        m.insert(
            "pxd",
            FileIcon {
                icon: '',
                color: "#5aa7e4",
            },
        );
        m.insert(
            "pxi",
            FileIcon {
                icon: '',
                color: "#5aa7e4",
            },
        );
        m.insert(
            "py",
            FileIcon {
                icon: '',
                color: "#ffbc03",
            },
        );
        m.insert(
            "pyc",
            FileIcon {
                icon: '',
                color: "#ffe291",
            },
        );
        m.insert(
            "pyd",
            FileIcon {
                icon: '',
                color: "#ffe291",
            },
        );
        m.insert(
            "pyi",
            FileIcon {
                icon: '',
                color: "#ffbc03",
            },
        );
        m.insert(
            "pyo",
            FileIcon {
                icon: '',
                color: "#ffe291",
            },
        );
        m.insert(
            "pyw",
            FileIcon {
                icon: '',
                color: "#5aa7e4",
            },
        );
        m.insert(
            "pyx",
            FileIcon {
                icon: '',
                color: "#5aa7e4",
            },
        );
        m.insert(
            "qm",
            FileIcon {
                icon: '',
                color: "#2596be",
            },
        );
        m.insert(
            "qml",
            FileIcon {
                icon: '',
                color: "#40cd52",
            },
        );
        m.insert(
            "qrc",
            FileIcon {
                icon: '',
                color: "#40cd52",
            },
        );
        m.insert(
            "qss",
            FileIcon {
                icon: '',
                color: "#40cd52",
            },
        );
        m.insert(
            "query",
            FileIcon {
                icon: '',
                color: "#90a850",
            },
        );
        m.insert(
            "r",
            FileIcon {
                icon: '󰟔',
                color: "#2266ba",
            },
        );
        m.insert(
            "rake",
            FileIcon {
                icon: '',
                color: "#701516",
            },
        );
        m.insert(
            "rar",
            FileIcon {
                icon: '',
                color: "#eca517",
            },
        );
        m.insert(
            "razor",
            FileIcon {
                icon: '󱦘',
                color: "#512bd4",
            },
        );
        m.insert(
            "rb",
            FileIcon {
                icon: '',
                color: "#701516",
            },
        );
        m.insert(
            "res",
            FileIcon {
                icon: '',
                color: "#cc3e44",
            },
        );
        m.insert(
            "resi",
            FileIcon {
                icon: '',
                color: "#f55385",
            },
        );
        m.insert(
            "rlib",
            FileIcon {
                icon: '',
                color: "#dea584",
            },
        );
        m.insert(
            "rmd",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "rproj",
            FileIcon {
                icon: '󰗆',
                color: "#358a5b",
            },
        );
        m.insert(
            "rs",
            FileIcon {
                icon: '',
                color: "#dea584",
            },
        );
        m.insert(
            "rss",
            FileIcon {
                icon: '',
                color: "#FB9D3B",
            },
        );
        m.insert(
            "sass",
            FileIcon {
                icon: '',
                color: "#f55385",
            },
        );
        m.insert(
            "sbt",
            FileIcon {
                icon: '',
                color: "#cc3e44",
            },
        );
        m.insert(
            "scad",
            FileIcon {
                icon: '',
                color: "#f9d72c",
            },
        );
        m.insert(
            "scala",
            FileIcon {
                icon: '',
                color: "#cc3e44",
            },
        );
        m.insert(
            "sc",
            FileIcon {
                icon: '',
                color: "#cc3e44",
            },
        );
        m.insert(
            "scm",
            FileIcon {
                icon: '󰘧',
                color: "#eeeeee",
            },
        );
        m.insert(
            "scss",
            FileIcon {
                icon: '',
                color: "#f55385",
            },
        );
        m.insert(
            "sh",
            FileIcon {
                icon: '',
                color: "#4d5a5e",
            },
        );
        m.insert(
            "sha1",
            FileIcon {
                icon: '󰕥',
                color: "#8c86af",
            },
        );
        m.insert(
            "sha224",
            FileIcon {
                icon: '󰕥',
                color: "#8c86af",
            },
        );
        m.insert(
            "sha256",
            FileIcon {
                icon: '󰕥',
                color: "#8c86af",
            },
        );
        m.insert(
            "sha384",
            FileIcon {
                icon: '󰕥',
                color: "#8c86af",
            },
        );
        m.insert(
            "sha512",
            FileIcon {
                icon: '󰕥',
                color: "#8c86af",
            },
        );
        m.insert(
            "sig",
            FileIcon {
                icon: 'λ',
                color: "#e37933",
            },
        );
        m.insert(
            "signature",
            FileIcon {
                icon: 'λ',
                color: "#e37933",
            },
        );
        m.insert(
            "skp",
            FileIcon {
                icon: '󰻫',
                color: "#839463",
            },
        );
        m.insert(
            "sldasm",
            FileIcon {
                icon: '󰻫',
                color: "#839463",
            },
        );
        m.insert(
            "sldprt",
            FileIcon {
                icon: '󰻫',
                color: "#839463",
            },
        );
        m.insert(
            "slim",
            FileIcon {
                icon: '',
                color: "#e34c26",
            },
        );
        m.insert(
            "sln",
            FileIcon {
                icon: '',
                color: "#854CC7",
            },
        );
        m.insert(
            "slvs",
            FileIcon {
                icon: '󰻫',
                color: "#839463",
            },
        );
        m.insert(
            "sml",
            FileIcon {
                icon: 'λ',
                color: "#e37933",
            },
        );
        m.insert(
            "so",
            FileIcon {
                icon: '',
                color: "#dcddd6",
            },
        );
        m.insert(
            "sol",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "spec.js",
            FileIcon {
                icon: '',
                color: "#cbcb41",
            },
        );
        m.insert(
            "spec.jsx",
            FileIcon {
                icon: '',
                color: "#20c2e3",
            },
        );
        m.insert(
            "spec.ts",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "spec.tsx",
            FileIcon {
                icon: '',
                color: "#1354bf",
            },
        );
        m.insert(
            "sql",
            FileIcon {
                icon: '',
                color: "#dad8d8",
            },
        );
        m.insert(
            "sqlite",
            FileIcon {
                icon: '',
                color: "#dad8d8",
            },
        );
        m.insert(
            "sqlite3",
            FileIcon {
                icon: '',
                color: "#dad8d8",
            },
        );
        m.insert(
            "srt",
            FileIcon {
                icon: '󰨖',
                color: "#ffb713",
            },
        );
        m.insert(
            "ssa",
            FileIcon {
                icon: '󰨖',
                color: "#ffb713",
            },
        );
        m.insert(
            "stl",
            FileIcon {
                icon: '󰆧',
                color: "#888888",
            },
        );
        m.insert(
            "strings",
            FileIcon {
                icon: '',
                color: "#2596be",
            },
        );
        m.insert(
            "ste",
            FileIcon {
                icon: '󰻫',
                color: "#839463",
            },
        );
        m.insert(
            "step",
            FileIcon {
                icon: '󰻫',
                color: "#839463",
            },
        );
        m.insert(
            "stp",
            FileIcon {
                icon: '󰻫',
                color: "#839463",
            },
        );
        m.insert(
            "styl",
            FileIcon {
                icon: '',
                color: "#8dc149",
            },
        );
        m.insert(
            "sub",
            FileIcon {
                icon: '󰨖',
                color: "#ffb713",
            },
        );
        m.insert(
            "sublime",
            FileIcon {
                icon: '',
                color: "#e37933",
            },
        );
        m.insert(
            "suo",
            FileIcon {
                icon: '',
                color: "#854CC7",
            },
        );
        m.insert(
            "sv",
            FileIcon {
                icon: '󰍛',
                color: "#019833",
            },
        );
        m.insert(
            "svelte",
            FileIcon {
                icon: '',
                color: "#ff3e00",
            },
        );
        m.insert(
            "svh",
            FileIcon {
                icon: '󰍛',
                color: "#019833",
            },
        );
        m.insert(
            "svg",
            FileIcon {
                icon: '󰜡',
                color: "#FFB13B",
            },
        );
        m.insert(
            "swift",
            FileIcon {
                icon: '',
                color: "#e37933",
            },
        );
        m.insert(
            "t",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "tbc",
            FileIcon {
                icon: '󰛓',
                color: "#1e5cb3",
            },
        );
        m.insert(
            "tcl",
            FileIcon {
                icon: '󰛓',
                color: "#1e5cb3",
            },
        );
        m.insert(
            "templ",
            FileIcon {
                icon: '',
                color: "#dbbd30",
            },
        );
        m.insert(
            "terminal",
            FileIcon {
                icon: '',
                color: "#31B53E",
            },
        );
        m.insert(
            "test.js",
            FileIcon {
                icon: '',
                color: "#cbcb41",
            },
        );
        m.insert(
            "test.jsx",
            FileIcon {
                icon: '',
                color: "#20c2e3",
            },
        );
        m.insert(
            "test.ts",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "test.tsx",
            FileIcon {
                icon: '',
                color: "#1354bf",
            },
        );
        m.insert(
            "tex",
            FileIcon {
                icon: '',
                color: "#3D6117",
            },
        );
        m.insert(
            "tf",
            FileIcon {
                icon: '',
                color: "#5F43E9",
            },
        );
        m.insert(
            "tfvars",
            FileIcon {
                icon: '',
                color: "#5F43E9",
            },
        );
        m.insert(
            "tgz",
            FileIcon {
                icon: '',
                color: "#eca517",
            },
        );
        m.insert(
            "tmux",
            FileIcon {
                icon: '',
                color: "#14ba19",
            },
        );
        m.insert(
            "toml",
            FileIcon {
                icon: '',
                color: "#9c4221",
            },
        );
        m.insert(
            "torrent",
            FileIcon {
                icon: '',
                color: "#44cda8",
            },
        );
        m.insert(
            "tres",
            FileIcon {
                icon: '',
                color: "#6d8086",
            },
        );
        m.insert(
            "ts",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "tscn",
            FileIcon {
                icon: '',
                color: "#6d8086",
            },
        );
        m.insert(
            "tsconfig",
            FileIcon {
                icon: '',
                color: "#FF8700",
            },
        );
        m.insert(
            "tsx",
            FileIcon {
                icon: '',
                color: "#1354bf",
            },
        );
        m.insert(
            "ttf",
            FileIcon {
                icon: '',
                color: "#ECECEC",
            },
        );
        m.insert(
            "twig",
            FileIcon {
                icon: '',
                color: "#8dc149",
            },
        );
        m.insert(
            "txz",
            FileIcon {
                icon: '',
                color: "#eca517",
            },
        );
        m.insert(
            "typoscript",
            FileIcon {
                icon: '',
                color: "#FF8700",
            },
        );
        m.insert(
            "txt",
            FileIcon {
                icon: '󰈙',
                color: "#89e051",
            },
        );
        m.insert(
            "ui",
            FileIcon {
                icon: '',
                color: "#015BF0",
            },
        );
        m.insert(
            "v",
            FileIcon {
                icon: '󰍛',
                color: "#019833",
            },
        );
        m.insert(
            "vala",
            FileIcon {
                icon: '',
                color: "#7239b3",
            },
        );
        m.insert(
            "vh",
            FileIcon {
                icon: '󰍛',
                color: "#019833",
            },
        );
        m.insert(
            "vhd",
            FileIcon {
                icon: '󰍛',
                color: "#019833",
            },
        );
        m.insert(
            "vhdl",
            FileIcon {
                icon: '󰍛',
                color: "#019833",
            },
        );
        m.insert(
            "vim",
            FileIcon {
                icon: '',
                color: "#019833",
            },
        );
        m.insert(
            "vsh",
            FileIcon {
                icon: '',
                color: "#5d87bf",
            },
        );
        m.insert(
            "vsix",
            FileIcon {
                icon: '',
                color: "#854CC7",
            },
        );
        m.insert(
            "vue",
            FileIcon {
                icon: '',
                color: "#8dc149",
            },
        );
        m.insert(
            "wasm",
            FileIcon {
                icon: '',
                color: "#5c4cdb",
            },
        );
        m.insert(
            "wav",
            FileIcon {
                icon: '',
                color: "#00afff",
            },
        );
        m.insert(
            "webm",
            FileIcon {
                icon: '',
                color: "#FD971F",
            },
        );
        m.insert(
            "webmanifest",
            FileIcon {
                icon: '',
                color: "#f1e05a",
            },
        );
        m.insert(
            "webp",
            FileIcon {
                icon: '',
                color: "#a074c4",
            },
        );
        m.insert(
            "webpack",
            FileIcon {
                icon: '󰜫',
                color: "#519aba",
            },
        );
        m.insert(
            "wma",
            FileIcon {
                icon: '',
                color: "#00afff",
            },
        );
        m.insert(
            "woff",
            FileIcon {
                icon: '',
                color: "#ECECEC",
            },
        );
        m.insert(
            "woff2",
            FileIcon {
                icon: '',
                color: "#ECECEC",
            },
        );
        m.insert(
            "wrl",
            FileIcon {
                icon: '󰆧',
                color: "#888888",
            },
        );
        m.insert(
            "wrz",
            FileIcon {
                icon: '󰆧',
                color: "#888888",
            },
        );
        m.insert(
            "wv",
            FileIcon {
                icon: '',
                color: "#00afff",
            },
        );
        m.insert(
            "wvc",
            FileIcon {
                icon: '',
                color: "#00afff",
            },
        );
        m.insert(
            "x",
            FileIcon {
                icon: '',
                color: "#599eff",
            },
        );
        m.insert(
            "xm",
            FileIcon {
                icon: '',
                color: "#519aba",
            },
        );
        m.insert(
            "xaml",
            FileIcon {
                icon: '󰙳',
                color: "#512bd4",
            },
        );
        m.insert(
            "xcf",
            FileIcon {
                icon: '',
                color: "#635b46",
            },
        );
        m.insert(
            "xcplayground",
            FileIcon {
                icon: '',
                color: "#e37933",
            },
        );
        m.insert(
            "xcstrings",
            FileIcon {
                icon: '',
                color: "#2596be",
            },
        );
        m.insert(
            "xls",
            FileIcon {
                icon: '󰈛',
                color: "#207245",
            },
        );
        m.insert(
            "xlsx",
            FileIcon {
                icon: '󰈛',
                color: "#207245",
            },
        );
        m.insert(
            "xml",
            FileIcon {
                icon: '󰗀',
                color: "#e37933",
            },
        );
        m.insert(
            "xpi",
            FileIcon {
                icon: '',
                color: "#ff1b01",
            },
        );
        m.insert(
            "xul",
            FileIcon {
                icon: '',
                color: "#e37933",
            },
        );
        m.insert(
            "xz",
            FileIcon {
                icon: '',
                color: "#eca517",
            },
        );
        m.insert(
            "yaml",
            FileIcon {
                icon: '',
                color: "#6d8086",
            },
        );
        m.insert(
            "yml",
            FileIcon {
                icon: '',
                color: "#6d8086",
            },
        );
        m.insert(
            "zig",
            FileIcon {
                icon: '',
                color: "#f69a1b",
            },
        );
        m.insert(
            "zip",
            FileIcon {
                icon: '',
                color: "#eca517",
            },
        );
        m.insert(
            "zsh",
            FileIcon {
                icon: '',
                color: "#89e051",
            },
        );
        m.insert(
            "zst",
            FileIcon {
                icon: '',
                color: "#eca517",
            },
        );
        m
    };
}

const DEFAULT_FILE_ICON: FileIcon = FileIcon {
    icon: '\u{f016}',
    color: "#7e8e91",
};

const DEFAULT_DIR_ICON: FileIcon = FileIcon {
    icon: '\u{f115}',
    color: "#7e8e91",
};

pub fn icon_for_file(file: &File<'_>) -> FileIcon {
    // if ICONS_MAP contains the file name or the file extension, return the icon
    if let Some(icon) = ICONS_MAP.get(file.name) {
        return icon.clone();
    } else if let Some(extension) = &file.ext {
        if let Some(icon) = ICONS_MAP.get(extension.as_str()) {
            return icon.clone();
        } else if let Some(icon) = ICONS_MAP.get(extension.to_lowercase().as_str()) {
            return icon.clone();
        } else {
            DEFAULT_FILE_ICON
        }
    } else if file.points_to_directory() {
        DEFAULT_DIR_ICON
    } else {
        DEFAULT_FILE_ICON
    }
}
