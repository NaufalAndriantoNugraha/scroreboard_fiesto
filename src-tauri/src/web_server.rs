use warp::Filter;

pub async fn run_http_server() {
    let route = warp::path("public_scoreboard_page").map(|| {
        // let ip = local_ip()
        //     .map(|ip| ip.to_string())
        //     .unwrap_or("0.0.0.0".into());
        // println!("IPPPPPPPPPPPP: {ip}");
        let local_ip = local_ip_address::local_ip().unwrap().to_string();
        warp::reply::html(generate_html(&local_ip))
    });

    println!("HTTP Server berjalan di http://0.0.0.0:8070/public_scoreboard_page");

    warp::serve(route).run(([0, 0, 0, 0], 8070)).await;
}

fn generate_html(ip: &str) -> String {
    format!(
        r#"
        <!doctype html>
<html>
 <head>
    <title>WebSocket Test</title>
    <meta charset="utf-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Martian+Mono:wght@100..800&display=swap" rel="stylesheet">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Oswald:wght@200..700&family=Urbanist:ital,wght@0,100..900;1,100..900&display=swap" rel="stylesheet">
    <style>
body {{
    font-family: "Martian Mono", monospace;
    font-size: 24px;
    box-sizing: border-box;
    margin: 0;
    /*color: #fff;*/
    height: 100vh;
    display: flex;
    font-weight: bold;
    align-items: center;
    }}
.box {{
    padding: 20px;
    width: 100%;
    display: flex;
    border: 0;
    flex-wrap: wrap;
    text-align: center;
    box-sizing: border-box;
    }}
.team-name {{
    text-transform: uppercase;
    margin-bottom: 20px;
    display: block;
    }}
.team-score {{
    font-size: 52pt;
    margin-bottom: 30px;
    display: block;
    font-family: "Oswald", sans-serif;
    line-height: 1.2;
    }}
.tim-terang {{
    width: 50%;
    }}
.tim-gelap {{
    width: 50%;
    }}
span#timer {{
    font-size: 42pt;
    margin: 30px 0;
    display: block;
    }}
.timer {{
    text-align: center;
    width: 100%;
    }}
    </style>
  </head>
  <body>
    <div class="box">
      <div class="tim-terang">
        <div class="team-name"><span id="team-a-name">-</span></div>
        <div class="team-score"><span id="team-a-score">0</span></div>
        <div>F <span id="team-a-foul">0</span></div>
        <div>T <span id="team-a-timeout">0</span></div>
      </div>
      <div class="tim-gelap">
        <div class="team-name"><span id="team-b-name">-</span></div>
        <div class="team-score"><span id="team-b-score">0</span></div>
        <div>F <span id="team-b-foul">0</span></div>
        <div>T <span id="team-b-timeout">0</span></div>
      </div>
      <div class="timer">
        <div class="timer"><span id="timer">0</span></div>
        <!--<div>Timeout: <span id="timeout">0</span></div>-->
        <div><span id="quarter">0</span></div>
      </div>
    </div>

    <script>
    function formatTimer(str) {{
      str = String(str)
      if (!str.includes(".")) return str;
    
      let [m, s] = str.split(".");
    
      // menit 1 digit → tambah 0 di depan
      if (m.length === 1) m = "0" + m;
    
      // detik 1 digit → tambah 0 di belakang → "0" -> "00", "2" -> "20"
      if (s.length === 1) s = s + "0";
    
      return m + "." + s;
    }}

      //const ws = new WebSocket("ws://72.61.140.101:8070");
     // const ws = new WebSocket("ws://ws.sportkit.club:8070");
// const ws = new WebSocket("ws://ws.sportkit.club");
const ws = new WebSocket("ws://{ip}:8071/ws");

      ws.onopen = () => {{
        console.log("Connected to WS server");
    
        // heartbeat pin
        setInterval(() => {{
          if (ws.readyState === WebSocket.OPEN) {{
            ws.send(JSON.stringify({{ type: "ping" }}));
            console.log("Ping sent");
    }}
    }}, 20000); // 20 detik
    }};
      ws.onmessage = (event) => {{
        console.log("Data:", event.data);
        try {{
          const data = JSON.parse(event.data);
          document.getElementById("team-a-name").innerText =
            data.team_a_name ?? "Tidak Muncul";
          document.getElementById("team-a-score").innerText =
            data.team_a_score ?? "Tidak Muncul";
          document.getElementById("team-a-foul").innerText =
            data.team_a_foul ?? "Tidak Muncul";
          document.getElementById("team-a-timeout").innerText =
            data.team_a_timeout ?? "Tidak Muncul";

          document.getElementById("team-b-name").innerText =
            data.team_b_name ?? "Tidak Muncul";
          document.getElementById("team-b-score").innerText =
            data.team_b_score ?? "Tidak Muncul";
          document.getElementById("team-b-foul").innerText =
            data.team_b_foul ?? "Tidak Muncul";
          document.getElementById("team-b-timeout").innerText =
            data.team_b_timeout ?? "";

         
            result = formatTimer(data.timer);
          
          document.getElementById("timer").innerText =
            result ?? "";
          // document.getElementById("timeout").innerText =
          //   data.timeout ?? "";
          

          document.getElementById("quarter").innerText =
            convertQuarter(data.quarter) ?? "";
    }} catch (e) {{
          console.log("Invalid JSON:", e);
    }}
    }};
      ws.onerror = (err) => {{
        console.log("WS Error:", err);
    }};
      ws.onclose = () => {{
        console.log("Disconnected");
    }};
      
      
      function convertQuarter(num) {{
            // Jika quarter 1–4 → Q1–Q4
            if (num >= 1 && num <= 4) {{
                return "Q" + num;
    }}
        
            // Jika lebih dari 4 → OT1, OT2, dst
            if (num > 4) {{
                return "OT" + (num - 4);
    }}
        
            // Jika tidak valid
            return "Q0";
    }}

    </script>
  </body>
</html>
    "#,
        ip = ip
    )
}
