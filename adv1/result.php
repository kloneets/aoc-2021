<?php

$data = array_filter(array_map(function ($w) {
  return $w ? (int) $w : null;
}, explode("\n", file_get_contents('data.csv'))));

$bigger = 0;
$last = 1000000;
foreach ($data as $w) {
  if ($w > $last) {
    $bigger++;
  }
  $last = $w;
}

var_dump($bigger);
