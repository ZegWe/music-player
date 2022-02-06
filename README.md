# A terminal music player written in the Rust language.

![running](./screenshots/running.png)
![playing](./screenshots/playing.png)

## Requirements

Configuration file path: C:\Users\xxx\\.config\music_player\config.yml

```yml
# Path to store music
music_database: ''
theme:
  list_title_color: '#ffaaff'
  list_title_page_color: '#ffb747'
  list_border_color: '#ffffff'
  list_music_color: '#eee4c4'
  list_folder_color: '#eee4c4'
  list_icon_color: '#f07178'
  list_selected_color: '#c3e88d'
  search_border_color: '#ffb747'
  search_icon_color: '#ec998b'
  search_font_color: '#eee4c4'
  command_font_color: '#eee4c4'
  command_border_color: '#c3eead'
  music_pic_color1: '#f07178'
  music_pic_color2: '#81a8fd'
  usage_color_left: '#beb2ec'
  usage_color_right: '#eee188'
  cut_off_rule_color: '#c3e88d'
  play_music_list_title_color: '#81a8fd'
  play_music_list_border_color: '#ffaaff'
  play_music_list_id_color: '#e0d7ca'
  play_music_list_duration_color: '#a9c34f'
  play_music_list_name_color: '#eee4c4'
  play_music_list_artist_color: '#b2e2e4'
  play_music_list_album_color: '#eee188'
  play_music_list_header_color: '#d15aa7'
  playing_music_border_color: '#81a8fd'
  playing_music_name_color: '#d8ce2e'
  gauge_color: '#cece68'
  gauge_border_color: '#abcc7e'
  gauge_label_color: '#fa4d70'
```

## Usage

### Browse

| Description                  | Event                  |
| ---------------------------- | ---------------------- |
| Exit program                 | q                      |
| Move selection down          | j \| \<Down Arrow Key> |
| Move selection up            | k \| \<Up Arrow Key>   |
| Move selection down 5 steps  | J                      |
| Move selection up 5 steps    | K                      |
| Move selection to the top    | g                      |
| Move selection to the bottom | G                      |
| Next page                    | n                      |
| Previous page                | N                      |
| Open folder                  | l                      |
| Back previous folder         | h                      |
| Into command mode            | :                      |
| Into search mode             | \|                     |
| Exit search or command mode  | \<Esc>                 |
| Pause the music              | \<Space>               |
| Add music to the paly list   | \<Enter>               |

### Command

| Description                                                               | Command           |
| ------------------------------------------------------------------------- | ----------------- |
| Add all the music in the current directory to the playlist                | all               |
| Removes the specified music from the playlist (Multiple can be specified) | rm \<music_id>    |
| Remove all music from the playlist                                        | clear \| cls      |
| Set music to play in order                                                | order \| od       |
| Set Set single loop                                                       | singlecycle \| sc |
