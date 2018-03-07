import subprocess
import discord
import sqlite3
import asyncio
import re

SECRET = "WRITE YOUR SECRET"

client = discord.Client()
voice = {}
channel = {}
settings = {}
playing = []
JTALK = '/usr/local/share/open_jtalk/open_jtalk_dic_utf_8-1.10'
# MEI_NORMAL = '/usr/local/share/hts_voice/mei/mei_normal.htsvoice'
# DEFAULT = '/usr/local/share/hts_voice/hts_voice_nitech_jp_atr503_m001-1.05/nitech_jp_atr503_m001.htsvoice'
# TOHOKU_NORMAL = '/usr/local/share/hts_voice/htsvoice-tohoku-f01/tohoku-f01-neutral.htsvoice'
# cmd = ('open_jtalk', '-x', JTALK, '-m', MEI_NORMAL, '-ow', '/mnt/ram/temp.wav')
VOICE = '/home/centos/discord_bot/read/htsvoice/{}.htsvoice'
VOICE_LIST = ['man', 'woman', 'mei_angry', 'mei_bashful', 'mei_happy', 'mei_sad',
              'tohoku', 'tohoku_angry', 'tohoku_happy', 'tohoku_sad',
              'endo', 'fuuki', 'giruko', 'hisei', 'homu', 'ichiri', 'ikuru', 'ikuto',
              'kanata', 'kono', 'mai', 'matsuo', 'nero', 'niji', 'otoko', 'rakan',
              'riyon', 'rou', 'sou', 'wamea', 'watashi', 'yoe']

help_message = '''
使えるコマンド
自分のいるボイスチャンネルに呼ぶ：`$$summon`
終了する：`$$bye`
声の設定：`$$setting [声の種類] [スピード:0.0-] [抑揚:0.0-] [声質a:0.0-1.0] [声質fm:0.0-]`
 例）`$$setting woman 1.0 1.0 0.5 1.5`
声の設定を確認する:`$$setting`

声の種類一覧
```
'man', 'woman',
'mei_angry', 'mei_bashful', 'mei_happy', 'mei_sad',
'tohoku', 'tohoku_angry', 'tohoku_happy', 'tohoku_sad',
'endo', 'fuuki', 'giruko', 'hisei', 'homu', 'ichiri',
'ikuru', 'ikuto', 'kanata', 'kono', 'mai', 'matsuo',
'nero', 'niji', 'otoko', 'rakan', 'riyon',
'rou', 'sou', 'wamea', 'watashi', 'yoe'
```
'''

# CREATE TABLE setting(user_id INTEGER primary key, voice_type TEXT, speed REAL, intonation REAL, timbre_a REAL, timbre_fm REAL);
conn = sqlite3.connect("read_setting.db")
conn.execute("CREATE TABLE IF NOT EXISTS setting(user_id INTEGER primary key, voice_type TEXT, speed REAL, intonation REAL, timbre_a REAL, timbre_fm REAL)")

counter = 0
FILE_NUM = 100

romaji = {
    'a': 'ア', 'i': 'イ', 'u': 'ウ', 'e': 'エ', 'o': 'オ',
    'ka': 'カ', 'ki': 'キ', 'ku': 'ク', 'ke': 'ケ', 'ko': 'コ',
    'sa': 'サ', 'shi': 'シ', 'su': 'ス', 'se': 'セ', 'so': 'ソ',
    'ta': 'タ', 'chi': 'チ', 'tu': 'ツ', 'te': 'テ', 'to': 'ト',
    'na': 'ナ', 'ni': 'ニ', 'nu': 'ヌ', 'ne': 'ネ', 'no': 'ノ',
    'ha': 'ハ', 'hi': 'ヒ', 'fu': 'フ', 'he': 'ヘ', 'ho': 'ホ',
    'ma': 'マ', 'mi': 'ミ', 'mu': 'ム', 'me': 'メ', 'mo': 'モ',
    'ya': 'ヤ', 'yu': 'ユ', 'yo': 'ヨ',
    'ra': 'ラ', 'ri': 'リ', 'ru': 'ル', 're': 'レ', 'ro': 'ロ',
    'wa': 'ワ', 'wo': 'ヲ', 'n': 'ン', 'vu': 'ヴ',
    'ga': 'ガ', 'gi': 'ギ', 'gu': 'グ', 'ge': 'ゲ', 'go': 'ゴ',
    'za': 'ザ', 'ji': 'ジ', 'zu': 'ズ', 'ze': 'ゼ', 'zo': 'ゾ',
    'da': 'ダ', 'di': 'ヂ', 'du': 'ヅ', 'de': 'デ', 'do': 'ド',
    'ba': 'バ', 'bi': 'ビ', 'bu': 'ブ', 'be': 'ベ', 'bo': 'ボ',
    'pa': 'パ', 'pi': 'ピ', 'pu': 'プ', 'pe': 'ペ', 'po': 'ポ',

    'kya': 'キャ', 'kyi': 'キィ', 'kyu': 'キュ', 'kye': 'キェ', 'kyo': 'キョ',
    'gya': 'ギャ', 'gyi': 'ギィ', 'gyu': 'ギュ', 'gye': 'ギェ', 'gyo': 'ギョ',
    'sha': 'シャ', 'shu': 'シュ', 'she': 'シェ', 'sho': 'ショ',
    'ja': 'ジャ', 'ju': 'ジュ', 'je': 'ジェ', 'jo': 'ジョ',
    'cha': 'チャ', 'chu': 'チュ', 'che': 'チェ', 'cho': 'チョ',
    'dya': 'ヂャ', 'dyi': 'ヂィ', 'dyu': 'ヂュ', 'dhe': 'デェ', 'dyo': 'ヂョ',
    'nya': 'ニャ', 'nyi': 'ニィ', 'nyu': 'ニュ', 'nye': 'ニェ', 'nyo': 'ニョ',
    'hya': 'ヒャ', 'hyi': 'ヒィ', 'hyu': 'ヒュ', 'hye': 'ヒェ', 'hyo': 'ヒョ',
    'bya': 'ビャ', 'byi': 'ビィ', 'byu': 'ビュ', 'bye': 'ビェ', 'byo': 'ビョ',
    'pya': 'ピャ', 'pyi': 'ピィ', 'pyu': 'ピュ', 'pye': 'ピェ', 'pyo': 'ピョ',
    'mya': 'ミャ', 'myi': 'ミィ', 'myu': 'ミュ', 'mye': 'ミェ', 'myo': 'ミョ',
    'rya': 'リャ', 'ryi': 'リィ', 'ryu': 'リュ', 'rye': 'リェ', 'ryo': 'リョ',
    'fa': 'ファ', 'fi': 'フィ', 'fe': 'フェ', 'fo': 'フォ',
    'wi': 'ウィ', 'we': 'ウェ',
    'va': 'ヴァ', 'vi': 'ヴィ', 've': 'ヴェ', 'vo': 'ヴォ',

    'kwa': 'クァ', 'kwi': 'クィ', 'kwu': 'クゥ', 'kwe': 'クェ', 'kwo': 'クォ',
    'kha': 'クァ', 'khi': 'クィ', 'khu': 'クゥ', 'khe': 'クェ', 'kho': 'クォ',
    'gwa': 'グァ', 'gwi': 'グィ', 'gwu': 'グゥ', 'gwe': 'グェ', 'gwo': 'グォ',
    'gha': 'グァ', 'ghi': 'グィ', 'ghu': 'グゥ', 'ghe': 'グェ', 'gho': 'グォ',
    'swa': 'スァ', 'swi': 'スィ', 'swu': 'スゥ', 'swe': 'スェ', 'swo': 'スォ',
    'zwa': 'ズヮ', 'zwi': 'ズィ', 'zwu': 'ズゥ', 'zwe': 'ズェ', 'zwo': 'ズォ',
    'twa': 'トァ', 'twi': 'トィ', 'twu': 'トゥ', 'twe': 'トェ', 'two': 'トォ',
    'dwa': 'ドァ', 'dwi': 'ドィ', 'dwu': 'ドゥ', 'dwe': 'ドェ', 'dwo': 'ドォ',
    'mwa': 'ムヮ', 'mwi': 'ムィ', 'mwu': 'ムゥ', 'mwe': 'ムェ', 'mwo': 'ムォ',
    'bwa': 'ビヮ', 'bwi': 'ビィ', 'bwu': 'ビゥ', 'bwe': 'ビェ', 'bwo': 'ビォ',
    'pwa': 'プヮ', 'pwi': 'プィ', 'pwu': 'プゥ', 'pwe': 'プェ', 'pwo': 'プォ',
    'phi': 'プィ', 'phu': 'プゥ', 'phe': 'プェ', 'pho': 'フォ',
}

romaji_asist = {
    'si': 'シ', 'ti': 'チ', 'hu': 'フ', 'zi': 'ジ',
    'sya': 'シャ', 'syu': 'シュ', 'syo': 'ショ',
    'tya': 'チャ', 'tyu': 'チュ', 'tyo': 'チョ',
    'cya': 'チャ', 'cyu': 'チュ', 'cyo': 'チョ',
    'jya': 'ジャ', 'jyu': 'ジュ', 'jyo': 'ジョ', 'pha': 'ファ',
    'qa': 'クァ', 'qi': 'クィ', 'qu': 'クゥ', 'qe': 'クェ', 'qo': 'クォ',

    'ca': 'カ', 'ci': 'シ', 'cu': 'ク', 'ce': 'セ', 'co': 'コ',
    'la': 'ラ', 'li': 'リ', 'lu': 'ル', 'le': 'レ', 'lo': 'ロ',

    'mb': 'ム', 'py': 'パイ', 'tho': 'ソ', 'thy': 'ティ', 'oh': 'オウ',
    'by': 'ビィ', 'cy': 'シィ', 'dy': 'ディ', 'fy': 'フィ', 'gy': 'ジィ',
    'hy': 'シー', 'ly': 'リィ', 'ny': 'ニィ', 'my': 'ミィ', 'ry': 'リィ',
    'ty': 'ティ', 'vy': 'ヴィ', 'zy': 'ジィ',

    'b': 'ブ', 'c': 'ク', 'd': 'ド', 'f': 'フ', 'g': 'グ', 'h': 'フ', 'j': 'ジ',
    'k': 'ク', 'l': 'ル', 'm': 'ム', 'p': 'プ', 'q': 'ク', 'r': 'ル', 's': 'ス',
    't': 'ト', 'v': 'ヴ', 'w': 'ゥ', 'x': 'クス', 'y': 'ィ', 'z': 'ズ',
}

words = {
    'OK': 'オーケィ', 'Bot': 'ボット', 'discha': 'ディスチャ',
    'Google': 'グーグル', 'google': 'グーグル', 'Twitter': 'ツイッター', 'twitter': 'ツイッター',
    'Skype': 'スカイプ', 'skype': 'スカイプ',
    'WiiU': 'ウィーユー', 'Switch': 'スウィッチ', 'PS4': 'ピーエスフォー', '3DS': 'スリーディーエス',
    'Minecraft': 'マインクラフト', 'minecraft': 'マインクラフト',
}

if not discord.opus.is_loaded():
    discord.opus.load_opus('/usr/local/lib/libopus.so')


@client.event
async def on_ready():
    global settings
    settings_from_db = conn.execute("SELECT * FROM setting")
    for s in settings_from_db:
        settings[str(s[0])] = [str(s[1]), str(s[2]),
                               str(s[3]), str(s[4]), str(s[5])]
    print('Logged in as')
    print(client.user.name)
    print(client.user.id)
    print('------')
    await client.change_presence(game=discord.Game(name="$$help|discha.net", url="https://discha.net"))


@client.event
async def on_voice_state_update(before, after):
    voice_channel = before.voice.voice_channel
    if voice_channel and (not after.voice.voice_channel or after.voice.voice_channel.id != voice_channel.id) \
            and len(voice_channel.voice_members) == 1 and voice_channel.server.id in voice:
        await voice[voice_channel.server.id].disconnect()
        del voice[voice_channel.server.id]
        del channel[voice_channel.server.id]


@client.event
async def on_message(message):
    if message.author.bot:
        return
    global voice
    global channel
    server_id = message.server.id
    if message.content == "$$help":
        await client.send_message(message.channel, help_message)
        return
    if message.content == "$$summon":
        if server_id not in voice:
            voice[server_id] = await client.join_voice_channel(message.author.voice_channel)
            channel[server_id] = message.channel.id
        return
    if message.content == "$$bye":
        await voice[server_id].disconnect()
        del voice[server_id]
        del channel[server_id]
        return
    if message.content == "$$setting":
        s = settings[message.author.id] if message.author.id in settings else [
            'man', '1.5', '2.0', '0.54', '3.5']
        await client.send_message(message.channel, "<@{}>さんの声の設定は`$$setting {} {} {} {} {}`です。".format(
            message.author.id, s[0], s[1], s[2], s[3], s[4]))
        return
    if message.content.startswith("$$setting "):
        comment = setting(message.content.split(" ")[1:], message.author.id)
        return await client.send_message(message.channel, comment)
    if server_id in voice and message.channel.id == channel[server_id]:
        try:
            player = voice[server_id].create_ffmpeg_player(create_wav(message))
            await wait_my_turn(server_id)
            await play(player)
        except Exception as e:
            print(e)
        finally:
            if server_id in playing:
                playing.remove(server_id)


async def wait_my_turn(server_id):
    count = 0
    while server_id in playing:
        await asyncio.sleep(0.1)
        count += 1
        if count > 600:
            break
    playing.append(server_id)


async def play(player):
    player.start()
    count = 0
    while player.is_playing():
        await asyncio.sleep(0.1)
        count += 1
        if count > 600:
            break


def create_wav(message):
    global counter
    file_name = "wav/{}.wav".format(counter % FILE_NUM)
    counter += 1
    s = settings[message.author.id] if settings.get(message.author.id) else [
        'woman', '1.0', '1.0', '0.5', '2.0']
    voice_type = VOICE.format(s[0])
    command = ('open_jtalk', '-x', JTALK, '-m', voice_type, '-ow', file_name,
               '-r', s[1], '-jf', s[2], '-a', s[3], '-fm', s[4])
    c = subprocess.Popen(command, stdin=subprocess.PIPE)
    c.stdin.write(clean_text(message).encode('utf-8'))
    c.stdin.close()
    c.wait(timeout=10)
    return file_name


def clean_text(message):
    result = message.content
    result = re.sub(r'https?://[\w/:%#\$&\?\(\)~\.=\+\-]+', "ユーアールエル", result)
    result = re.sub(r'w{2,}', " わらわら ", result)
    result = re.sub(r'ｗ{2,}', " わらわら ", result)
    result = re.sub(r'(?i)discord', "ディスコード", result)
    result = re.sub(r'<:.*>', " ", result)
    if message.mentions:
        match = re.search(r'<@(\d+)>', result)
        while match:
            result = re.sub(r'<@(\d+)>', get_member(message.mentions,
                                                    match.expand(r'\1')), result, count=1)
            match = re.search(r'<@(\d+)>', result)
    if message.mentions:
        match = re.search(r'<@!(\d+)>', result)
        while match:
            result = re.sub(r'<@!(\d+)>', get_member(message.mentions,
                                                     match.expand(r'\1')), result, count=1)
            match = re.search(r'<@!(\d+)>', result)
    if message.role_mentions:
        match = re.search(r'<@&(\d+)>', result)
        while match:
            result = re.sub(r'<@&(\d+)>', get_role(message.role_mentions,
                                                   match.expand(r'\1')), result, count=1)
            match = re.search(r'<@&(\d+)>', result)
    if message.channel_mentions:
        result = re.sub(r'<#(\d+)>', "チャンネル", result)
    result = re_words.sub(lambda x: words[x.group(0)], result)
    result = _romaji2katakana(result)
    if len(result) > 64:
        result = result[:60] + "。イカリャク"
    return result


def get_member(members, member_id):
    for member in members:
        if member.id == member_id:
            if member.nick:
                return member.nick
            else:
                return member.name
    return ""


def get_role(roles, role_id):
    for role in roles:
        if role.id == role_id:
            return role.name
    return ""


def setting(s, user_id):
    if len(s) == 5 and float(s[1]) >= 0.0 and float(s[2]) >= 0.0 \
            and float(s[3]) >= 0.0 and float(s[4]) >= 0.0 and float(s[3]) <= 1.0 and s[0] in VOICE_LIST:
        settings[user_id] = s
        conn.execute("replace into setting values(?,?,?,?,?,?)",
                     (int(user_id), s[0], s[1], s[2], s[3], s[4]))
        conn.commit()
        return "<@{}>さんの声を`$$setting {} {} {} {} {}`に設定しました。".format(
            user_id, s[0], s[1], s[2], s[3], s[4])
    elif float(s[3]) > 1.0:
        return "3番目の数字は0.0-1.0の間です。"
    else:
        return help_message


romaji_dict = {}
for tbl in romaji, romaji_asist:
    for k, v in tbl.items():
        romaji_dict[k] = v

romaji_keys = romaji_dict.keys()
romaji_keys = sorted(romaji_keys, key=lambda x: len(x), reverse=True)

re_roma2kana = re.compile("|".join(map(re.escape, romaji_keys)))
rx_mba = re.compile("m(b|p)([aiueo])")
rx_xtu = re.compile(r"([bcdfghjklmpqrstvwxyz])\1")
rx_a__ = re.compile(r"([aiueo])\1")

re_words = re.compile("|".join(map(re.escape, words.keys())))


def _romaji2katakana(text):
    result = text.lower()
    result = rx_mba.sub(r"ン\1\2", result)
    result = rx_xtu.sub(r"ッ\1", result)
    result = rx_a__.sub(r"\1ー", result)
    return re_roma2kana.sub(lambda x: romaji_dict[x.group(0)], result)


client.run(SECRET)
