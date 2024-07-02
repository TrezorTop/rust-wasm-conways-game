export const fps = new class {
  fps: HTMLDivElement | null;
  frames: number[];
  lastFrameTimeStamp: number;

  constructor() {
    this.fps = document.querySelector('.fps');
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
  }

  render() {
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const fps = 1 / delta * 1000;

    // save only the latest 100 frames
    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    let min = Infinity;
    let max = -Infinity;
    let sum = 0;

    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i];
      min = Math.min(min, this.frames[i]);
      max = Math.max(max, this.frames[i]);
    }

    const mean = sum / this.frames.length;

    if (this.fps) {
      this.fps.textContent = `FPS:\nlatest = ${Math.round(fps)}\navg of last 100 = ${Math.round(mean)}\nmin of last 100 = ${Math.round(min)}\nmax of last 100 = ${Math.round(max)}`;
    }
  }
}