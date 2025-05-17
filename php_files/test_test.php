<?php

$code = file_get_contents("short.php"); // 100.000 Zeichen

$START = hrtime(true);
$tokens = token_get_all($code);
$end = hrtime(true);

printTokens($tokens, $code);
//printOnlyStringsAndFollowing($tokens, $code);

$duration_ms = ($end - $START) / 1_000_000;
echo "Lexing took: {$duration_ms} ms\n";
echo "Number of Tokens found: " . count($tokens) . "\n\n";

/*foreach ($tokens as $index => $token) {
    if (is_array($token)) {
        [$id, $text, $line] = $token;
        $name = token_name($id);
        $short = strlen($text) > 40 ? substr($text, 0, 37) . '...' : $text;
        printf("[%03d] %-20s (Line %d): \"%s\"\n", $index, $name, $line, addcslashes($short, "\n\r\t"));
    } else {
        printf("[%03d] %-20s: \"%s\"\n", $index, 'SINGLE_CHAR', $token);
    }
}*/

function printTokens(array $tokens, string $source): void
{
    foreach ($tokens as $token) {
        if (is_array($token)) {
            [$id, $text, $line] = $token;
            $tagName = token_name($id);
        } else {
            $id = null;
            $text = $token;
            $tagName = 'T_CHAR';
        }

        // Escape Steuerzeichen sichtbar machen
        $escaped = addcslashes($text, "\0..\37\\\"");
        echo "$tagName =>\n";
        echo "\t\"$escaped\"\n\n";
    }
}

function printOnlyStringsAndFollowing(array $tokens, string $source): void
{
    $count = count($tokens);
    $i = 0;

    while ($i < $count) {
        $token = $tokens[$i];

        if (is_array($token)) {
            [$id, $text, $line] = $token;
            $tagName = token_name($id);
        } else {
            $id = null;
            $text = $token;
            $tagName = 'T_CHAR';
        }

        // Falls der Token ein double-quoted String ist
        if ($id === T_CONSTANT_ENCAPSED_STRING && str_starts_with($text, '"')) {
            // Print diesen Token + 4 danach
            for ($j = 0; $j < 5 && $i + $j < $count; $j++) {
                $t = $tokens[$i + $j];
                if (is_array($t)) {
                    [$tid, $ttxt] = $t;
                    $tname = token_name($tid);
                } else {
                    $tid = null;
                    $ttxt = $t;
                    $tname = 'T_CHAR';
                }

                $escaped = addcslashes($ttxt, "\0..\37\\\"");
                echo "$tname =>\n";
                echo "\t\"$escaped\"\n\n";
            }
            $i += 5; // weiter nach dem Block
        } else {
            $i++;
        }
    }
}



