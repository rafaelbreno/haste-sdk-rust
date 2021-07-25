import { LeaderboardInstance } from '@haste-sdk/domain/src';
import { Haste } from '@haste-sdk/sdk';
import { GameEngine } from '../../game/gameEngine';
export function logoutListener(_1, haste: Haste, engine: GameEngine) {
  // eslint-disable-next-line @typescript-eslint/no-floating-promises
  haste.game.score(engine.currentPlay, new LeaderboardInstance(''), 0);
}
