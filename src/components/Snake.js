import React, {Component} from 'react'
import styled from 'react-emotion'

const delay = ms => new Promise(resolve => setTimeout(resolve, ms))

const Canvas = styled.canvas`
  box-shadow: rgba(0, 0, 0, 0.05) 0px 0px 25px;
  align-self: center;
`

export default class Snake extends Component {
  componentDidMount() {
    document.addEventListener('keydown', this.handleInput)
    this.update()
  }

  handleInput = e => {
    switch (e.key) {
      case 'ArrowLeft':
        this.xv = -1
        this.yv = 0
        break
      case 'ArrowUp':
        this.xv = 0
        this.yv = -1
        break
      case 'ArrowRight':
        this.xv = 1
        this.yv = 0
        break
      case 'ArrowDown':
        this.xv = 0
        this.yv = 1
        break
    }
  }

  render = () => (
    <Canvas width={500} height={500} innerRef={c => (this.c = c)} />
  )
}
