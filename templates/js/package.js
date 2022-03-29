$(".readmore").each(function () {
    var $this = $(this),
        $lis = $this.children(),
        $a = $(`<a href='javascript:void(0)'>Less</a>`)
    if ($lis.length > 5) {
      $this.after($a);
      $a.click(function () {
        $lis.slice(5).toggle();
        $a.html($a.html() === "More" ? "Less" : "More")
      }).click();
    }
  });