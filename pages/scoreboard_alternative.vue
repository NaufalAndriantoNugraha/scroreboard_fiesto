<template>
  <link
    rel="stylesheet"
    href="https://cdn.jsdelivr.net/npm/bulma@1.0.4/css/bulma.min.css"
  />
  <div class="container">
    <h1>Cara Mengakses Tampilan Alternatif Scoreboard</h1>
    <p>
      Berikut merupakan langkah-langkah untuk mengakses tampilan alternatif
      scoreboard:
    </p>
    <ol>
      <li>
        Pada komputer Anda silahkan buka Control Panel, di Control Panel klik
        <span class="important">System Security</span>, setelah itu
        <span class="important">Windows Defender Firewall</span>, dan di sebelah
        kiri klik
        <span class="important">Turn Windows Defender Firewall on or off</span>,
        setelah itu pilih
        <span class="important"
          >Turn off windows defender firewall (not recommended)</span
        >
        di kedua opsi.
      </li>
      <li>
        Setelah mematikan firewall pada komputer Anda, silahkan buka terminal
        atau shell yang tersedia di komputer Anda. Untuk sistem operasi Windows,
        Anda dapat menggunakan PowerShell, atau Command Prompt.
      </li>
      <li>
        Setelah membuka terminal atau shell, silahkan ketik
        <span class="important">ipconfig</span> dan tekan enter. Setelah
        memasukkan perintah tersebut, pada layar terminal atau shell Anda akan
        muncul sebuah teks yang menunjukkan konfigurasi IP pada komputer Anda.
      </li>
      <li>
        Setelah itu cari teks
        <span class="important">Wireless LAN adapeter Wi-Fi</span>, dan di bawah
        bagian tersebut cari teks <span class="important">IPv4 Address</span>.
        Catat teks di sebelah kanan <span class="important">IPv4 Address</span>.
      </li>
      <li>
        Setelah mencatat alamat IPv4, pastikan aplikasi telah berjalan dan
        menunjukkan tampilan utama scoreboard.
      </li>
      <li>
        Apabila tampilan utama scoreboard telah terlihat, silahkan buka peramban
        yang Anda gunakan, dan ketik
        <span class="important"
          >http://{teks yang Anda catat}:8070/public_scoreboard_page</span
        >.
        <em
          >Perlu diketahui ketika mengetikkan di peramban tidak perlu
          mengikutkan tanda "{", dan "}", serta pastikan protokol yang digunakan
          adalah http bukan https</em
        >.
      </li>
      <li>
        Setelah mengikuti semua langkah-langkah dengan benar, halaman pada
        peramban Anda akan muncul tampilan alternatif scoreboard.
      </li>
    </ol>
  </div>
</template>

<style scoped>
.container {
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

li,
p {
  text-align: justify;
  line-height: 30px;
  letter-spacing: 0.3px;
}

h1 {
  font-weight: bold;
  font-size: 1.5rem;
  text-align: center;
}

ol {
  list-style-type: decimal !important;
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding-left: 20px;
}

.important {
  background-color: cadetblue;
  color: white;
  padding: 5px;
  border-radius: 0.25rem;
  font-family: monospace;
  font-size: 1rem;
}
</style>

<script lang="ts"></script>
