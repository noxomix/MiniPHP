<?php

echo "=== Standard Escapes ===\n";

echo "Newline: [Line1\nLine2]\n";
echo "Tab: [Start\tEnd]\n";
echo "Carriage return: [A\rB]\n";
echo "Vertical tab: [A\vB]\n";
echo "Escape: [\e]\n";
echo "Form feed: [A\fB]\n";
echo "Double quote: [\"]\n";
echo "Backslash: [\\]\n";
echo "Dollar sign: [\$var]\n";

echo "\n=== Octal Escapes ===\n";

echo "Null byte: [\0]\n";
echo "Bell (octal 7): [\07]\n";
echo "Octal 123 (should be S): [\123]\n";

echo "\n=== Hexadecimal Escapes ===\n";

echo "Hex 41 (should be A): [\x41]\n";
echo "Hex 7A (should be z): [\x7A]\n";

echo "\n=== Unicode Escapes (PHP 7+) ===\n";

echo "Euro sign: [\u{20AC}]\n";
echo "Grinning face emoji: [\u{1F600}]\n";
echo "Latin capital AE: [\u{00C6}]\n";

echo "\n=== Mixed Content ===\n";

echo "Mix: Line1\nTab\tOctal:\101 Hex:\x42 Unicode:\u{43} End\n";

echo "\n=== Edge cases ===\n";

// Invalid escape (should be literal)
echo "Invalid escape \\q remains as: [\\q]\n";

// Escaped backslash before escape (real \n)
echo "Double escaped newline: [\\n]\n";

echo "[\07]";     // ✅ Oktal 07 → ASCII BEL (Glocke)
echo "[\078]";    // ✅ \07 → BEL, dann '8' normal
echo "[\0x]";     // ✅ \0 → NULL-Byte, dann 'x'
echo "[\09]";     // ✅ \0 → NULL, dann '9'

echo "\n=== Finished ===\n";

echo "=== Sichtbare Ausgabe ===\n";
echo "[\08]\n";
echo "[\09]\n";
echo "[\0x]\n";
echo "[\012]\n"; // gültig (max 3 Oktalziffern) → \012 = 10, dann '3'

echo "\n=== Byte-Werte (ord) ===\n";

$strs = [
    "\\08" => "\08",
    "\\09" => "\09",
    "\\0x" => "\0x",
    "\\0123" => "\0123",
];

foreach ($strs as $label => $s) {
    echo "$label → ";
    foreach (str_split($s) as $c) {
        printf("0x%02X ", ord($c));
    }
    echo "\n";
}
