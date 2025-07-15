package app

import "github.com/mt-shihab26/termodoro/internal/timer"

func (app *App) nextSession() {
	app.session.NextSession()
	app.timer = timer.New(app.session.GetDuration())
	app.timer.Start()
}
