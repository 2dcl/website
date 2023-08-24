async function updateList() {
  const response = await fetch("/scenes.rss");
  const body = await response.text();
  fetch("/scenes.rss")
    .then(response => response.text())
    .then(str => new window.DOMParser().parseFromString(str, "text/xml"))
    .then(data => {
      console.log(data);
      let buildDate = data.getElementsByTagName('lastBuildDate')[0];
      let lastBuildDate = document.getElementById('last-build-date');
      lastBuildDate.textContent = buildDate.textContent;

      let items = data.getElementsByTagName('item');
      let table = document.getElementById('scene-table');

      for(const item of items) {
        const row = document.createElement("tr");
        const name = document.createElement("td");
        name.textContent = item.getElementsByTagName("title")[0].textContent;
        row.appendChild(name);

        const link = document.createElement("td");
        const uri =  item.getElementsByTagName("link")[0].textContent
        // const a = document.createElement("a");
        // a.setAttribute("href", uri);
        let parcel = uri.split('/').slice(-2);
        // a.innerText = `(${parcel[0]},${parcel[1]})`;
        // link.appendChild(a);
        link.textContent = `(${parcel[0]},${parcel[1]})`;
        row.appendChild(link);

        const pubDate = document.createElement("td");
        pubDate.innerText = item.getElementsByTagName("pubDate")[0].textContent;
        row.appendChild(pubDate);

        table.appendChild(row);
      }
    });
}

document.addEventListener("DOMContentLoaded", () => {
  let container = document.getElementById('content');

  updateList();
});

