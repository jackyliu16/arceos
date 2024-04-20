use crate::eth::Eth;
use bcm2711_regs::genet::umac::Cmd;

use crate::eth::Bcm54213peHal;

impl<'rx, 'tx, A: Bcm54213peHal> Eth<'rx, 'tx, A> {
    pub(crate) fn netif_start(&mut self) {
        self.dev.umac.cmd.modify(Cmd::TxEn::Set + Cmd::RxEn::Set);
    }
}
