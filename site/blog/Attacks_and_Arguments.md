# Arguments and Attacks Against Cryptocurrency

_23 September 2021_

---

Most of the media surrounding Bitcoin, Ethereum, and other similar projects is overwhelmingly positive. This may be due to people's heavy financial and emotional investments in the crypto space, or the real value in the technology. In any case, the disproportionate amount of positivity means that in order to understand the reasons _not_ to be a proponent of the technology, one needs to dedicate a search specifically for them.

## Energy consumption

The most notorious fault in Bitcoin is the energy consumption. Power usage comes from the Proof of Work (PoW) process, in which miners compete to be the first one to guess a correct answer to hash function. The amount of electricity consumed in the mining process is so huge that some comparisons to other power consumers are almost comical. For example, if the mining pools for Bitcoin formed their own country, it would rank 61st in the world in terms of power usage. [^1]

<figure>
	<a href="https://digiconomist.net/bitcoin-energy-consumption">
		<img src="/images/Bitcoin_Energy_Usage.png">
		<figcaption><em>Courtesy of Digiconomist</em></figcaption>
	</a>
</figure>

In addition to the electrical power required to mine Bitcoin, large amounts of water is needed to cool the machines used for mining. The manufacturing of the components used for these machines requires rare metals, the mining of which is an environmental hazard.

As Wei Dai established in his [b-money proposal](http://www.weidai.com/bmoney.txt), the computation required to solve PoW problems must "have no value, practical or intellectual," so that it is ensured that miners incurred a financial cost in producing the coins.[^2]

Renewable energy does not really solve this problem, since cheaper energy makes mining more profitable and thus increases mining activity, decreasing the electrical supply available to the rest of the world.

## Does not actually qualify as a currency

There are three requirements for something to be considered a currency. It must be a:
- medium of exchange
- unit of account
- store of value

Bitcoin does not satisfy any of these requirements.[^3] On the first count, high transaction costs and slow speed (which will be covered later) mean it isn't suitable for any day-to-day purchases. In addition, almost all of its traffic comes from investors trading, not from buying goods. 

The price of Bitcoin fluctuates wildly, disqualify it form the second and third requirements. No vendor will list the price of goods in Bitcoin, since they would have to change the price constantly. Although some Bitcoin maximalists tout its indisputability as making it a good store of value, but that is not what it means to hold value. For example, people would rather hold USD than Bitcoin, because the dollar is very likely to be worth a dollar in a month, whereas the value of Bitcoin may go zero in less time than that. 

## Unusable in everyday life

One historical barrier to the usage of cryptocurrency is a lack of infrastructure. Even if someone wants to use crypto, they need to research how to buy it and set up a wallet. Even with the recent trend of [Bitcoin ATMs](https://www.bitcoin.com/bitcoin-atm/), very few vendors accept cryptocurrency, mostly because there is no practical way to formally accept it, such as a card reader.

Even once one does get set up, it can be cumbersome to use. The high transaction costs and confirmation times are by design, and minimizing them is not always a purely positive thing. Some projects are trying to create a secure blockchain without high fees, but such technology is unproven.

Perhaps the biggest obstacle to mainstream usage is that a major behavioral shift is required to use cryptocurrency effectively. The authors of _Blockchain Revolution_ explain, 

>"Many people today rely on their bank or cerdit card company... when they make an accounting error, forget their passwords, or lose their wallets. Most people... aren't in the habit of backing up their money on a flash drive or a second device, securing their passwords so they needn't rely on a service provider's password reset function, or keeping these backups in seperate locations so that, if they lose their computer... in a house fire, they don't lose their money. Without this discipline, they might as well stuff their mattress with cash." [^4]

For now, the technology is only really suitable for enthusiasts who can tolerate a large amount of "geekspeak" and inefficiency. 

## Does not complement human society

As humans, we like to make rules, and we like to bend those rules just as much. We can easily rationalize this, but to a program running on a computer, any situation outside of the parameters is inconceivable.

Credit and debit cards have become the new default for payment systems. Visa, Paypal and the like are all centralized services run by a designated authority. Proponents of cryptocurrency claim, factually, that payments can be reversed in these systems. This is not as bad as they claim, and it actually happens all the time. What is one to do if their card gets stolen? With an immutable blockchain, there is no way for the theft to be rectified. 

## Privacy

When Bitcoin was first introduced, it was fairly anonymous. In the Facebook and Google era we currently occupy, Bitcoin addresses are nicknames at best; almost any Bitcoin address can be associated with a person accurately. As dystopian as that sounds, any entity who wants your information (like advertisers) can get it anyway, and find more than what is recorded on the blockchain.

Privacy in the modern age is a much bigger issue than cryptocurrency. I plan on devoting a journal just to this issue.

This makes 'digital identity' proposals, which would record public records on the blockchain, even more tricky. What information should be public, and what should be private? This is not always a clear line, and the discussion includes less about right vs wrong and more about our current social relationship with privacy.

## Criminal Activity

This is a subset of the privacy issue. Bitcoin, especially, is being used by criminals for ransom payments, black market purchases, and more. The extent to which it is used is widely debated because of varying methods for analyzing the ledger. 

If blockchain can help Google create targeted ads for us, could it help us track criminals? Is it worth sacrificing privacy for safety? Is Bitcoin more or less traceable than cash? These are not easy questions, and I will address these questions in my journal specifically on privacy.

## Lessons

Are all of these fundamental fallacies of cryptocurrency and blockchain, or simply obstacles to be overcome? That will be answered soon, or not at all. 

In my opinion, the best way to test this would be practical experiments, like [Project Jasper](https://www.bankofcanada.ca/research/digital-currencies-and-fintech/projects/). Hopefully, with more tangible evidence, we can see if the optimistic claims and the opposing arguments hold up. 

### Refrences

---

[^1]: _Digiconomist_. "Bitcoin Energy Consumption Index." 13 Sept. 2021, [digiconomist.net/bitcoin-energy-consumption](https://digiconomist.net/bitcoin-energy-consumption).

[^2]: Dai, Wei. "b-money." Wei Dai, Nov. 1998, [weidai.com/bmoney.txt](http://www.weidai.com/bmoney.txt).

[^3]: Yermack, David. "Is Bitcoin a Real Currency? An economic appraisal." _NBER Working Paper Series_, Dec. 2013. 

[^4]: Tapscott, Don, and Alex Tapscott. _Blockchain Revolution: How the Technology Behind Bitcoin and and Other Cryptocurrencies Is Changing the World_. Portfolio, 2018. 
