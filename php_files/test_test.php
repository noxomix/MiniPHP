<?php

$code = file_get_contents("bigfile.php"); // 100.000 Zeichen

$START = hrtime(true);
$tokens = token_get_all($code);
$end = hrtime(true);

$duration_ms = ($end - $START) / 1_000_000;
echo "Lexing took: {$duration_ms} ms\n";