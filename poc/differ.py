import io
import json

import asts
import click
from termcolor import colored

language_map = {
    "c": asts.ASTLanguage.C,
    "cpp": asts.ASTLanguage.Cpp,
    "java": asts.ASTLanguage.Java,
    "python": asts.ASTLanguage.Python,
    "rust": asts.ASTLanguage.Rust,
    "ts": asts.ASTLanguage.TypescriptTs,
    "tsx": asts.ASTLanguage.TypescriptTsx,
}


def compress_path(ast_path: list):
    """
    >>> compress_path([["CHILDREN", 5], "BODY", ["CHILDREN", 0], "VALUE", "FUNCTION", "FIELD"])
    '5/body/0/value/function/field'
    """
    segs = []
    for seg in ast_path:
        if len(seg) == 2:
            if seg[0] == "CHILDREN":
                segs.append(str(seg[1]))
            else:
                raise Exception("unhandled:", seg)
        else:
            segs.append(seg.lower())

    return "/".join(segs)


def decompress_path(compressed_path: str):
    """
    >>> decompress_path('5/body/0/value/function/field')
    [['CHILDREN', 5], 'BODY', ['CHILDREN', 0], 'VALUE', 'FUNCTION', 'FIELD']
    """
    segs = compressed_path.split("/")
    out = []
    for seg in segs:
        try:
            number = int(seg)
            out.append(["CHILDREN", number])
        except:
            out.append(seg.upper())

    return out


class LocationInformation:
    def __init__(self, root: asts.AST, ast: asts.AST):
        self.source_ranges = ast.ast_source_ranges()[0][1]
        self.path = compress_path(asts.AST.ast_path(root, ast))
        self.source_text = ast.source_text
        self.ast = ast

    def __repr__(self):
        return json.dumps({
            "source_ranges": self.source_ranges,
            "path": self.path,
            "source_text": self.source_text
        })


class SimpleDiff:
    def __init__(self, old: LocationInformation, new: LocationInformation):
        self.old = old
        self.new = new

        self.path = old.path if old.path == new.path else None

    def __repr__(self):
        return json.dumps({
            "path": self.path,
            "old": self.old.source_text,
            "new": self.new.source_text
        })


def recursive_diff_internal(parsed, parsed2, current, current2, diffs: list):
    if not current.children or not current2.children:
        if current.source_text != current2.source_text:
            simple_diff = SimpleDiff(LocationInformation(parsed, current), LocationInformation(parsed2, current2))
            diffs.append(simple_diff)
        return

    for idx, child in enumerate(current.children):
        if idx >= len(current2.children):
            # maybe there is a diff here?
            print("errata:", idx, child, current2.children)
            continue

        other = current2.children[idx]
        recursive_diff_internal(parsed, parsed2, child, other, diffs)


def recursive_diff(parsed, parsed2, diffs: list):
    for idx, child in enumerate(parsed.children):
        if idx >= len(parsed2.children):
            # maybe there is a diff here?
            print("errata:", idx, child, parsed2.children)
            continue

        other = parsed2.children[idx]
        recursive_diff_internal(parsed, parsed2, child, other, diffs)


def apply(language, parsed2, diffs: list[SimpleDiff]):
    for change in diffs:
        if not change.path:
            raise Exception("paths do not match for simple diff:", change)

        path = decompress_path(change.path)
        parsed2 = asts.AST.replace(parsed2, path, change.new.ast)

    return parsed2


@click.command()
@click.argument('path')
@click.argument('path2')
@click.argument('language')
def main(path: str, path2: str, language: str):
    language = language.lower()
    if language not in language_map:
        raise Exception(f"language {language} not supported: supported: {sorted(list(language_map.keys()))}")

    parsed = asts.AST.from_file(path, language_map[language])
    parsed2 = asts.AST.from_file(path2, language_map[language])
    diffs = []
    recursive_diff(parsed, parsed2, diffs)

    changed = apply(language, parsed, diffs)
    print(diffs)
    print(changed.source_text)


if __name__ == "__main__":
    main()
