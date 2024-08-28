$today = (get-date -format "dd")
if ($today -eq "28" -or $today -eq "11") {
  push-location $psscriptroot
  $fullday = $(get-date -format "yyyymmdd")
  $last = $(get-content "$psscriptroot/utopia.txt")
  write-host $last
  if ($last -ne $fullday) {
    write-host "sending utopia gazette"
    invoke-expression "$psscriptroot/utopia.exe -- montpellier avignon"
    py $psscriptroot/mail_attachment.py 
    if ($?) { write-output $fullday > "$psscriptroot/utopia.txt" } 
    write-host "done"
  }
  else {
    write-host "already done"
  }
  pop-location
}
else {
  write-host "u not day"
}
