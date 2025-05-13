<?php

$code = file_get_contents("bigfile.php"); // 100.000 Zeichen

$START = hrtime(true);
$tokens = token_get_all($code);
$end = hrtime(true);

$duration_ms = ($end - $START) / 1_000_000;
echo "Lexing took: {$duration_ms} ms\n";
echo "Number of Tokens found: " . count($tokens) . "\n\n";

foreach ($tokens as $index => $token) {
    if (is_array($token)) {
        [$id, $text, $line] = $token;
        $name = token_name($id);
        $short = strlen($text) > 40 ? substr($text, 0, 37) . '...' : $text;
        printf("[%03d] %-20s (Line %d): \"%s\"\n", $index, $name, $line, addcslashes($short, "\n\r\t"));
    } else {
        printf("[%03d] %-20s: \"%s\"\n", $index, 'SINGLE_CHAR', $token);
    }
}

