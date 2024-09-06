type PlayImageMessage = {
  remotePath: string
  text: string
  width: number
  height: number
}

type PlayVideoMessage = {
  remotePath: string
  text: string
}

type JoinMessage = {
  clientCount: number
}

