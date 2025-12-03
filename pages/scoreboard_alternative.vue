<template>
  <div class="container">
    <h1>Cara mengakses tampilan alternatif scoreboard</h1>
    <p>
      Berikut merupakan cara untuk mengakses tampilan alternatif scoreboard:
    </p>
    <ol>
      <li>
        Pada komputer Anda silahkan buka control panel, di control panel klik
        "System Security", setelah itu "Windows Defender Firewall", dan
        disebelah kiri terdapat opsi "Turn Windows Defender Firewall on or off",
        setelah itu pilih "Turn off windows defender firewall (not recommended)"
        di kedua opsi.
      </li>
      <li>
        Setelah mematikan firewall pada kommputer Anda, silahkan buka terminal
        atau shell yang tersedia di komputer Anda. Untuk sistem operasi Windows,
        Anda dapat menggunakan PowerShell, atau Command Propmt.
      </li>
      <li>
        Setelah membuka terminal atau shell, silahkan ketik ipconfig dan tekan
        Enter. Setelah memasukkan perintah tersebut, pada layar terminal atau
        shell Anda akan muncul sebuah teks yang menunjukkan konfigurasi IP pada
        komputer Anda.
      </li>
      <li>
        Setelah itu cari teks "Wireless LAN adapeter Wi-Fi", dan di bawah bagian
        tersebut cari teks "IPv4 Address". Catat teks di sebelah kanan "IPv4
        Address".
      </li>
      <li>
        Setelah mencatat alamat IPv4, pastikan aplikasi telah berjalan dan
        menunjukkan tampilan utama scoreboard.
      </li>
      <li>
        Apabila tampilan utama scoreboard telah terlihat, silahkan buka peramban
        yang Anda gunakan, dan ketik "http://{teks yang Anda
        catat}:8070/public_scoreboard_page". Perlu diketahui ketika mengetikkan
        di peramban tidak perlu mengikutkan tanda "{", dan "}".
      </li>
      <li>
        Apabila Anda mengikuti semua langkah dengan benar, halaman pada peramban
        Anda akan muncul tampilan alternatif scoreboard.
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
  letter-spacing: 0.25px;
}

h1 {
  font-weight: bold;
  font-size: 1.5rem;
}

ol {
  list-style-type: decimal !important;
  display: flex;
  flex-direction: column;
  gap: 5px;
  padding-left: 20px;
}
</style>

<script lang="ts"></script>
