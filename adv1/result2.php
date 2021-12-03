<?php

$data = array_filter(array_map(function ($w) {
  return $w ? (int) $w : null;
}, explode("\n", file_get_contents('data.csv'))));

$summedData = [];

$i = 0;
$sum = 0;
foreach ($data as $k => $d) {
  for ($j = 0; $j < 3; $j++) {
    if (isset($data[$k + $j])) {
      $sum += $data[$k + $j];
    }
  }
  $summedData[$i] = $sum;
  $i++;
  $sum = 0;
}

$bigger = 0;
$last = 100000;
foreach ($summedData as $w) {
  if ($w > $last) {
    $bigger++;
  }
  $last = $w;
}

var_dump($summedData[0], $summedData[1], $summedData[2], $bigger);
